use super::read;
use crate::auth;
use crate::discovery::{RestDescription, RestMethod};
use crate::error::GwsError;
use crate::executor::{self, AuthMethod, BodyValidationPolicy, PaginationConfig};
use crate::formatter::OutputFormat;
use crate::helpers::modelarmor::SanitizeConfig;
use clap::{Arg, ArgMatches, Command};
use serde_json::{json, Value};

pub(super) fn command() -> Command {
    let document = || {
        Arg::new("document")
            .long("document")
            .help("Document ID")
            .required(true)
            .value_name("ID")
    };
    let mut cmd = Command::new("+suggest").about("[Helper] Create and manage Docs suggestions");
    cmd = cmd.subcommand(
        Command::new("insert")
            .about("Insert text as a suggestion")
            .arg(document())
            .arg(text_arg())
            .arg(Arg::new("tab-id").long("tab-id").value_name("ID")),
    );
    cmd = cmd.subcommand(
        Command::new("replace")
            .about("Replace one exact text run as a suggestion")
            .arg(document())
            .arg(
                Arg::new("find")
                    .long("find")
                    .help("Exact text to replace")
                    .required(true)
                    .value_name("TEXT"),
            )
            .arg(text_arg())
            .arg(Arg::new("tab-id").long("tab-id").value_name("ID")),
    );
    cmd = cmd.subcommand(
        Command::new("delete-text")
            .about("Propose deleting a document range")
            .arg(document())
            .arg(index_arg("start-index"))
            .arg(index_arg("end-index"))
            .arg(Arg::new("tab-id").long("tab-id").value_name("ID")),
    );
    cmd = cmd.subcommand(
        Command::new("list")
            .about("List suggestions with document context")
            .arg(document())
            .arg(Arg::new("params").long("params").value_name("JSON")),
    );
    for action in ["accept", "reject", "delete"] {
        cmd = cmd.subcommand(
            Command::new(action)
                .about(format!("{} an existing suggestion", capitalize(action)))
                .arg(document())
                .arg(
                    Arg::new("suggestion-id")
                        .long("suggestion-id")
                        .required(true)
                        .value_name("ID"),
                ),
        );
    }
    cmd.after_help(
        "EXAMPLES:\n  gws docs +suggest insert --document DOC_ID --text 'Suggested text'\n  gws docs +suggest replace --document DOC_ID --find 'old' --text 'new'\n  gws docs +suggest list --document DOC_ID\n  gws docs +suggest accept --document DOC_ID --suggestion-id SUGGESTION_ID\n\nTIPS:\n  Suggestion writes are a Google Workspace Developer Preview feature.\n  The helper opts into unknown preview fields internally; raw commands remain strict.\n  Use --dry-run to preview insert, delete-text, accept, reject, and delete requests.\n  replace reads the document to locate exactly one matching text run before writing.",
    )
}

fn text_arg() -> Arg {
    Arg::new("text")
        .long("text")
        .help("Text to insert")
        .required(true)
        .value_name("TEXT")
}

fn index_arg(name: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .help("UTF-16 document index")
        .required(true)
        .value_parser(clap::value_parser!(i32))
}

fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub(super) async fn handle(
    doc: &RestDescription,
    matches: &ArgMatches,
    sanitize: &SanitizeConfig,
) -> Result<(), GwsError> {
    let (action, action_matches) = matches
        .subcommand()
        .ok_or_else(|| GwsError::Validation("docs +suggest requires an action".into()))?;
    if action == "list" {
        return handle_list(doc, action_matches, sanitize).await;
    }

    let (params, body, method) = match action {
        "insert" => {
            let method = batch_update_method(doc)?;
            let body = build_insert_body(action_matches)?;
            (document_params(action_matches)?, body, method)
        }
        "replace" => {
            let method = batch_update_method(doc)?;
            let body = build_replace_body(doc, action_matches, sanitize).await?;
            (document_params(action_matches)?, body, method)
        }
        "delete-text" => {
            let method = batch_update_method(doc)?;
            let body = build_delete_text_body(action_matches)?;
            (document_params(action_matches)?, body, method)
        }
        "accept" | "reject" | "delete" => {
            let method = batch_update_method(doc)?;
            let body = build_suggestion_action_body(action, action_matches)?;
            (document_params(action_matches)?, body, method)
        }
        _ => {
            return Err(GwsError::Validation(format!(
                "Unknown suggestion action: {action}"
            )))
        }
    };

    execute_suggestion_write(doc, method, &params, &body, sanitize, action_matches).await
}

async fn handle_list(
    doc: &RestDescription,
    matches: &ArgMatches,
    sanitize: &SanitizeConfig,
) -> Result<(), GwsError> {
    let output = read::run(doc, matches, sanitize, async {
        auth::get_token(&["https://www.googleapis.com/auth/documents.readonly"])
            .await
            .map_err(|e| GwsError::Auth(format!("Docs auth failed: {e}")))
    })
    .await?;
    println!("{output}");
    Ok(())
}

async fn execute_suggestion_write(
    doc: &RestDescription,
    method: &RestMethod,
    params: &str,
    body: &str,
    sanitize: &SanitizeConfig,
    matches: &ArgMatches,
) -> Result<(), GwsError> {
    let dry_run = matches.get_flag("dry-run");
    let scopes: Vec<&str> = crate::select_scope(&method.scopes).into_iter().collect();
    let token = if dry_run {
        None
    } else {
        Some(
            auth::get_token(&scopes)
                .await
                .map_err(|e| GwsError::Auth(format!("Docs auth failed: {e}")))?,
        )
    };
    executor::execute_method_with_policy(
        doc,
        method,
        Some(params),
        Some(body),
        token.as_deref(),
        if token.is_some() {
            AuthMethod::OAuth
        } else {
            AuthMethod::None
        },
        None,
        None,
        dry_run,
        &PaginationConfig::default(),
        sanitize.template.as_deref(),
        &sanitize.mode,
        &OutputFormat::default(),
        false,
        BodyValidationPolicy::AllowUnknownFields,
    )
    .await
    .map(|_| ())
}

fn batch_update_method(doc: &RestDescription) -> Result<&RestMethod, GwsError> {
    doc.resources
        .get("documents")
        .and_then(|resource| resource.methods.get("batchUpdate"))
        .ok_or_else(|| GwsError::Discovery("Method 'documents.batchUpdate' not found".into()))
}

fn document_params(matches: &ArgMatches) -> Result<String, GwsError> {
    let document = matches
        .get_one::<String>("document")
        .ok_or_else(|| GwsError::Validation("Document ID is required".into()))?;
    crate::validate::validate_resource_name(document)?;
    Ok(json!({"documentId": document}).to_string())
}

fn tab_id(matches: &ArgMatches) -> Option<&str> {
    matches.get_one::<String>("tab-id").map(String::as_str)
}

fn insert_location(matches: &ArgMatches) -> Value {
    let mut location = json!({"segmentId": ""});
    if let Some(tab_id) = tab_id(matches) {
        location["tabId"] = json!(tab_id);
    }
    location
}

fn build_insert_body(matches: &ArgMatches) -> Result<String, GwsError> {
    let text = matches
        .get_one::<String>("text")
        .ok_or_else(|| GwsError::Validation("Text is required".into()))?;
    Ok(json!({
        "requests": [{"insertText": {"text": text, "endOfSegmentLocation": insert_location(matches)}}],
        "writeControl": {"writeMode": "SUGGEST"}
    })
    .to_string())
}

async fn build_replace_body(
    doc: &RestDescription,
    matches: &ArgMatches,
    sanitize: &SanitizeConfig,
) -> Result<String, GwsError> {
    if matches.get_flag("dry-run") {
        return Err(GwsError::Validation(
            "replace cannot use --dry-run because it must read the document to locate the unique match"
                .into(),
        ));
    }
    let document = matches.get_one::<String>("document").unwrap();
    let find = matches.get_one::<String>("find").unwrap();
    if find.is_empty() {
        return Err(GwsError::Validation(
            "find must not be empty when replacing text".into(),
        ));
    }
    let read_matches = read_command_matches(document)?;
    let normalized = read::run(doc, &read_matches, sanitize, async {
        auth::get_token(&["https://www.googleapis.com/auth/documents.readonly"])
            .await
            .map_err(|e| GwsError::Auth(format!("Docs auth failed: {e}")))
    })
    .await?;
    let normalized_value: Value = serde_json::from_str(&normalized)
        .map_err(|e| GwsError::Validation(format!("Invalid normalized Docs response: {e}")))?;
    let revision_id = normalized_value
        .get("revisionId")
        .and_then(Value::as_str)
        .filter(|revision| !revision.is_empty())
        .ok_or_else(|| {
            GwsError::Validation(
                "Docs response did not include a revisionId; refusing an unprotected replacement"
                    .into(),
            )
        })?;
    let (tab_id, start, end) = find_unique_text_run(&normalized, find, tab_id(matches))?;
    let text = matches.get_one::<String>("text").unwrap();
    let mut delete_range = json!({"startIndex": start, "endIndex": end});
    let mut insert_location = json!({"index": start});
    if let Some(tab_id) = tab_id {
        delete_range["tabId"] = json!(tab_id);
        insert_location["tabId"] = json!(tab_id);
    }
    Ok(json!({
        "requests": [
            {"deleteContentRange": {"range": delete_range}},
            {"insertText": {"text": text, "location": insert_location}}
        ],
        "writeControl": {"writeMode": "SUGGEST", "requiredRevisionId": revision_id}
    })
    .to_string())
}

fn read_command_matches(document: &str) -> Result<ArgMatches, GwsError> {
    read::command()
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .action(clap::ArgAction::SetTrue),
        )
        .try_get_matches_from(["+read", "--document", document])
        .map_err(|e| GwsError::Validation(format!("Unable to prepare document read: {e}")))
}

fn find_unique_text_run(
    document: &str,
    needle: &str,
    requested_tab: Option<&str>,
) -> Result<(Option<String>, i32, i32), GwsError> {
    if needle.is_empty() {
        return Err(GwsError::Validation(
            "Text to replace must not be empty".into(),
        ));
    }
    let mut matches = Vec::new();
    find_text_runs(document, needle, requested_tab, &mut matches);
    match matches.as_slice() {
        [(tab_id, start, end)] => Ok((tab_id.clone(), *start, *end)),
        [] => Err(GwsError::Validation(
            "Text to replace was not found in one text run".into(),
        )),
        _ => Err(GwsError::Validation(
            "Text to replace matched more than once".into(),
        )),
    }
}

fn find_text_runs(
    value: &str,
    needle: &str,
    requested_tab: Option<&str>,
    matches: &mut Vec<(Option<String>, i32, i32)>,
) {
    let Ok(value) = serde_json::from_str::<Value>(value) else {
        return;
    };
    if let Some(tabs) = value.get("tabs").and_then(Value::as_array) {
        for tab in tabs {
            walk_tab_text_runs(tab, needle, requested_tab, matches);
        }
    }
}

fn walk_tab_text_runs(
    tab: &Value,
    needle: &str,
    requested_tab: Option<&str>,
    matches: &mut Vec<(Option<String>, i32, i32)>,
) {
    let tab_id = tab.get("tabId").and_then(Value::as_str).map(String::from);
    if requested_tab.is_none_or(|requested| tab_id.as_deref() == Some(requested)) {
        if let Some(blocks) = tab.get("blocks") {
            walk_text_runs(blocks, needle, tab_id.clone(), matches);
        }
    }
    if let Some(children) = tab.get("childTabs").and_then(Value::as_array) {
        for child in children {
            walk_tab_text_runs(child, needle, requested_tab, matches);
        }
    }
}

fn walk_text_runs(
    value: &Value,
    needle: &str,
    current_tab: Option<String>,
    matches: &mut Vec<(Option<String>, i32, i32)>,
) {
    match value {
        Value::Object(object) => {
            let tab = object
                .get("tabId")
                .and_then(Value::as_str)
                .map(String::from)
                .or(current_tab);
            if object.get("type").and_then(Value::as_str) == Some("text") {
                if let (Some(text), Some(start), Some(run_end)) = (
                    object.get("text").and_then(Value::as_str),
                    object.get("startIndex").and_then(Value::as_i64),
                    object.get("endIndex").and_then(Value::as_i64),
                ) {
                    for offset in text.char_indices().filter_map(|(offset, _)| {
                        text[offset..].starts_with(needle).then_some(offset)
                    }) {
                        let start = start + text[..offset].encode_utf16().count() as i64;
                        let end = start + needle.encode_utf16().count() as i64;
                        if end <= run_end {
                            matches.push((tab.clone(), start as i32, end as i32));
                        }
                    }
                }
            }
            for child in object.values() {
                walk_text_runs(child, needle, tab.clone(), matches);
            }
        }
        Value::Array(array) => {
            for child in array {
                walk_text_runs(child, needle, current_tab.clone(), matches);
            }
        }
        _ => {}
    }
}

fn build_delete_text_body(matches: &ArgMatches) -> Result<String, GwsError> {
    let start = *matches.get_one::<i32>("start-index").unwrap();
    let end = *matches.get_one::<i32>("end-index").unwrap();
    if start < 0 || end <= start {
        return Err(GwsError::Validation(
            "end-index must be greater than start-index and both must be non-negative".into(),
        ));
    }
    let mut range = json!({"startIndex": start, "endIndex": end});
    if let Some(tab_id) = tab_id(matches) {
        range["tabId"] = json!(tab_id);
    }
    Ok(json!({
        "requests": [{"deleteContentRange": {"range": range}}],
        "writeControl": {"writeMode": "SUGGEST"}
    })
    .to_string())
}

fn build_suggestion_action_body(action: &str, matches: &ArgMatches) -> Result<String, GwsError> {
    let suggestion_id = matches.get_one::<String>("suggestion-id").unwrap();
    let request = match action {
        "accept" => json!({"acceptSuggestion": {"suggestionId": suggestion_id}}),
        "reject" => json!({"rejectSuggestion": {"suggestionId": suggestion_id}}),
        "delete" => json!({"deleteSuggestion": {"suggestionId": suggestion_id}}),
        _ => {
            return Err(GwsError::Validation(format!(
                "Unknown suggestion action: {action}"
            )))
        }
    };
    Ok(json!({"requests": [request]}).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_body_uses_suggest_mode() {
        let matches = command()
            .try_get_matches_from(["+suggest", "insert", "--document", "doc", "--text", "hello"])
            .unwrap();
        let body = build_insert_body(matches.subcommand_matches("insert").unwrap()).unwrap();
        let body: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(body["writeControl"]["writeMode"], "SUGGEST");
        assert_eq!(body["requests"][0]["insertText"]["text"], "hello");
    }

    #[test]
    fn delete_text_rejects_invalid_range() {
        let matches = command()
            .try_get_matches_from([
                "+suggest",
                "delete-text",
                "--document",
                "doc",
                "--start-index",
                "4",
                "--end-index",
                "4",
            ])
            .unwrap();
        let error =
            build_delete_text_body(matches.subcommand_matches("delete-text").unwrap()).unwrap_err();
        assert!(error.to_string().contains("end-index"));
    }

    #[test]
    fn suggestion_action_uses_requested_id() {
        let matches = command()
            .try_get_matches_from([
                "+suggest",
                "accept",
                "--document",
                "doc",
                "--suggestion-id",
                "s1",
            ])
            .unwrap();
        let body =
            build_suggestion_action_body("accept", matches.subcommand_matches("accept").unwrap())
                .unwrap();
        let body: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(
            body["requests"][0]["acceptSuggestion"]["suggestionId"],
            "s1"
        );
        assert!(body.get("writeControl").is_none());
    }

    #[test]
    fn finds_unique_match_using_utf16_indices() {
        let document = r#"{"tabs":[{"tabId":"tab-1","blocks":[{"elements":[{"type":"text","text":"A😀BC","startIndex":5,"endIndex":10}]}]}]}"#;
        assert_eq!(
            find_unique_text_run(document, "😀B", None).unwrap(),
            (Some("tab-1".into()), 6, 9)
        );
    }

    #[test]
    fn rejects_empty_replacement_text() {
        let document = r#"{"tabs":[{"tabId":"tab-1","blocks":[{"elements":[{"type":"text","text":"hello","startIndex":1,"endIndex":6}]}]}]}"#;
        assert!(find_unique_text_run(document, "", None).is_err());
    }

    #[test]
    fn rejects_duplicate_matches_within_one_text_run() {
        let document = r#"{"tabs":[{"tabId":"tab-1","blocks":[{"elements":[{"type":"text","text":"foo foo","startIndex":1,"endIndex":8}]}]}]}"#;
        assert!(find_unique_text_run(document, "foo", None).is_err());
    }

    #[test]
    fn filters_replacement_matches_by_tab_id() {
        let document = r#"{"tabs":[{"tabId":"tab-1","blocks":[{"elements":[{"type":"text","text":"foo","startIndex":1,"endIndex":4}]}]},{"tabId":"tab-2","blocks":[{"elements":[{"type":"text","text":"foo","startIndex":1,"endIndex":4}]}]}]}"#;
        assert_eq!(
            find_unique_text_run(document, "foo", Some("tab-2")).unwrap(),
            (Some("tab-2".into()), 1, 4)
        );
    }

    #[test]
    fn rejects_overlapping_matches() {
        let document = r#"{"tabs":[{"tabId":"tab-1","blocks":[{"elements":[{"type":"text","text":"aaa","startIndex":1,"endIndex":4}]}]}]}"#;
        assert!(find_unique_text_run(document, "aa", None).is_err());
    }

    #[test]
    fn finds_text_in_child_tabs() {
        let document = r#"{"tabs":[{"tabId":"root","blocks":[],"childTabs":[{"tabId":"child","blocks":[{"elements":[{"type":"text","text":"child text","startIndex":1,"endIndex":11}]}],"childTabs":[]}]}]}"#;
        assert_eq!(
            find_unique_text_run(document, "child", Some("child")).unwrap(),
            (Some("child".into()), 1, 6)
        );
    }
}
