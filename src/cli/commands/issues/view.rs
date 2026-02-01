use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_issue_detail, print_issue_json};

pub async fn view_issue(client: &SentryClient, issue_id: &str) -> Result<()> {
    let issue = client.get_issue(issue_id).await?;

    // Fetch the latest event to get stack trace and details
    let event = client.get_latest_event(issue_id).await.ok();

    match get_format() {
        OutputFormat::Json => print_issue_json(&issue),
        OutputFormat::Table | OutputFormat::Compact => print_issue_detail(&issue, event.as_ref()),
    }

    Ok(())
}
