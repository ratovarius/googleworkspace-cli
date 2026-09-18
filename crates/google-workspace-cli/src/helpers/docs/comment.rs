use crate::auth;
use crate::discovery::{RestDescription, RestMethod};
use crate::error::GwsError;
use crate::executor::{self, AuthMethod, BodyValidationPolicy, PaginationConfig};
use crate::formatter::OutputFormat;
use crate::helpers::modelarmor::SanitizeConfig;
use clap::{Arg, ArgMatches, Command};
use serde_json::json;

pub(super) fn command() -> Command {
    Command::new("+comment")
        .about("[Helper] Create a comment anchored to document text")
        .subcommand(
            Command::new("create")
                .about("Create a comment on a document range")
                .arg(
                    Arg::new("document")
                        .long("document")
                        .help("Document ID")
                        .required(true)
                        .value_name("ID"),
                )
                .arg(
                    Arg::new("text")
                        .long("text")
                        .help("Comment text")
                        .required(true)
                        .value_name("TEXT"),
                )
                .arg(index_arg("start-index"))
                .arg(index_arg("end-index"))
                .arg(Arg::new("tab-id").long("tab-id").value_name("ID")),
        )
        .after_help(
            "EXAMPLES:\n  gws docs +comment create --document DOC_ID --text 'Please review this.' --start-index 1 --end-index 20\n\nTIPS:\n  Indexes are UTF-16 document indexes.\n  Comment creation is a Google Workspace Developer Preview feature.\n  Use --dry-run to validate without authentication or sending the request.",
        )
}

fn index_arg(name: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .help("UTF-16 document index")
        .required(true)
        .value_parser(clap::value_parser!(i32))
}

pub(super) async fn handle(
    doc: &RestDescription,
    matches: &ArgMatches,
    sanitize: &SanitizeConfig,
) -> Result<(), GwsError> {
    let (action, action_matches) = matches
        .subcommand()
        .ok_or_else(|| GwsError::Validation("docs +comment requires an action".into()))?;
    if action != "create" {
        return Err(GwsError::Validation(format!(
            "Unknown comment action: {action}"
        )));
    }
    let method = batch_update_method(doc)?;
    let params = document_params(action_matches)?;
    let body = build_comment_create_body(action_matches)?;
    let dry_run = action_matches.get_flag("dry-run");
    let scopes: Vec<&str> = method.scopes.iter().map(String::as_str).collect();
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
        Some(&params),
        Some(&body),
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

fn build_comment_create_body(matches: &ArgMatches) -> Result<String, GwsError> {
    let text = matches
        .get_one::<String>("text")
        .ok_or_else(|| GwsError::Validation("Comment text is required".into()))?;
    let start = *matches
        .get_one::<i32>("start-index")
        .ok_or_else(|| GwsError::Validation("start-index is required".into()))?;
    let end = *matches
        .get_one::<i32>("end-index")
        .ok_or_else(|| GwsError::Validation("end-index is required".into()))?;
    if start < 0 || end <= start {
        return Err(GwsError::Validation(
            "end-index must be greater than start-index and both must be non-negative".into(),
        ));
    }
    let mut range = json!({"startIndex": start, "endIndex": end});
    if let Some(tab_id) = matches.get_one::<String>("tab-id") {
        range["tabId"] = json!(tab_id);
    }
    Ok(json!({
        "requests": [{"insertComment": {"content": text, "range": range}}]
    })
    .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn comment_create_body_uses_requested_range_and_content() {
        let matches = Command::new("+comment")
            .arg(Arg::new("document").long("document"))
            .arg(Arg::new("text").long("text"))
            .arg(
                Arg::new("start-index")
                    .long("start-index")
                    .value_parser(clap::value_parser!(i32)),
            )
            .arg(
                Arg::new("end-index")
                    .long("end-index")
                    .value_parser(clap::value_parser!(i32)),
            )
            .arg(Arg::new("tab-id").long("tab-id"))
            .try_get_matches_from([
                "+comment",
                "--document",
                "doc",
                "--text",
                "Review this",
                "--start-index",
                "1",
                "--end-index",
                "10",
            ])
            .unwrap();
        let body = build_comment_create_body(&matches).unwrap();
        let body: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(
            body["requests"][0]["insertComment"]["content"],
            "Review this"
        );
        assert_eq!(
            body["requests"][0]["insertComment"]["range"]["startIndex"],
            1
        );
        assert_eq!(
            body["requests"][0]["insertComment"]["range"]["endIndex"],
            10
        );
    }

    #[test]
    fn comment_create_rejects_non_positive_range() {
        let matches = Command::new("+comment")
            .arg(Arg::new("text").long("text"))
            .arg(
                Arg::new("start-index")
                    .long("start-index")
                    .value_parser(clap::value_parser!(i32)),
            )
            .arg(
                Arg::new("end-index")
                    .long("end-index")
                    .value_parser(clap::value_parser!(i32)),
            )
            .arg(Arg::new("tab-id").long("tab-id"))
            .try_get_matches_from([
                "+comment",
                "--text",
                "Review this",
                "--start-index",
                "10",
                "--end-index",
                "10",
            ])
            .unwrap();
        assert!(build_comment_create_body(&matches).is_err());
    }
}
