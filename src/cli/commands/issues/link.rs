use crate::api::models::CreateExternalIssue;
use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::{Result, SentryCliError};
use crate::output::{get_format, print_external_issue_json, print_success};

/// Parse a Linear URL into (project, identifier).
///
/// Expected format: `https://linear.app/{workspace}/issue/{KEY-123}/...`
fn parse_linear_url(url: &str) -> Option<(String, String)> {
    let path = url.strip_prefix("https://linear.app/")?;
    let segments: Vec<&str> = path.split('/').collect();
    // segments: [workspace, "issue", "KEY-123", ...]
    if segments.len() >= 3 && segments[1] == "issue" {
        let identifier = segments[2];
        let project = identifier.rsplit_once('-').map(|(p, _)| p)?;
        Some((project.to_string(), identifier.to_string()))
    } else {
        None
    }
}

pub async fn link_issues(
    client: &SentryClient,
    issue_ids: Vec<String>,
    url: &str,
    explicit_project: Option<String>,
    explicit_identifier: Option<String>,
    integration: &str,
) -> Result<()> {
    let (project, identifier) = match (explicit_project, explicit_identifier) {
        (Some(p), Some(id)) => (p, id),
        (explicit_p, explicit_id) => {
            let parsed = parse_linear_url(url);
            let project = explicit_p
                .or_else(|| parsed.as_ref().map(|(p, _)| p.clone()))
                .ok_or_else(|| {
                    SentryCliError::Validation(
                        "Could not infer --project from URL. Provide it explicitly.".to_string(),
                    )
                })?;
            let identifier = explicit_id
                .or_else(|| parsed.as_ref().map(|(_, id)| id.clone()))
                .ok_or_else(|| {
                    SentryCliError::Validation(
                        "Could not infer --identifier from URL. Provide it explicitly.".to_string(),
                    )
                })?;
            (project, identifier)
        }
    };

    let installations = client.list_sentry_app_installations().await?;
    let installation = installations
        .iter()
        .find(|i| i.app.slug == integration)
        .ok_or_else(|| {
            SentryCliError::NotFound(format!(
                "No installation found for integration '{integration}'"
            ))
        })?;

    for issue_id in &issue_ids {
        let numeric_id: u64 = issue_id.parse().map_err(|_| {
            SentryCliError::Validation(format!(
                "Issue ID '{issue_id}' must be numeric for external issue linking"
            ))
        })?;

        let body = CreateExternalIssue {
            issue_id: numeric_id,
            web_url: url.to_string(),
            project: project.clone(),
            identifier: identifier.clone(),
        };

        let external_issue = client
            .create_external_issue(&installation.uuid, &body)
            .await?;

        match get_format() {
            OutputFormat::Json => print_external_issue_json(&external_issue),
            OutputFormat::Table | OutputFormat::Compact => {
                print_success(&format!(
                    "Linked issue {} to {} ({})",
                    issue_id, identifier, external_issue.id
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_linear_url_standard() {
        let url = "https://linear.app/megacrit/issue/PRG-1234/some-title";
        let result = parse_linear_url(url);
        assert_eq!(result, Some(("PRG".to_string(), "PRG-1234".to_string())));
    }

    #[test]
    fn test_parse_linear_url_no_title_suffix() {
        let url = "https://linear.app/megacrit/issue/PRG-5678";
        let result = parse_linear_url(url);
        assert_eq!(result, Some(("PRG".to_string(), "PRG-5678".to_string())));
    }

    #[test]
    fn test_parse_linear_url_different_project() {
        let url = "https://linear.app/acme/issue/ENG-42/fix-bug";
        let result = parse_linear_url(url);
        assert_eq!(result, Some(("ENG".to_string(), "ENG-42".to_string())));
    }

    #[test]
    fn test_parse_linear_url_not_linear() {
        assert_eq!(parse_linear_url("https://example.com/issue/123"), None);
    }

    #[test]
    fn test_parse_linear_url_missing_issue_segment() {
        assert_eq!(
            parse_linear_url("https://linear.app/megacrit/PRG-1234"),
            None
        );
    }

    #[test]
    fn test_parse_linear_url_no_hyphen_in_identifier() {
        // An identifier without a hyphen means we can't split project from number
        assert_eq!(
            parse_linear_url("https://linear.app/megacrit/issue/nohyphen"),
            None
        );
    }
}
