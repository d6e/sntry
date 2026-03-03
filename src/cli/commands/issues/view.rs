use crate::api::models::IssueDetail;
use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_issue_detail, print_issue_detail_json};

pub async fn view_issue(client: &SentryClient, issue_id: &str) -> Result<()> {
    let issue = client.get_issue(issue_id).await?;
    let event = client.get_latest_event(issue_id).await.ok();

    let detail = IssueDetail {
        issue,
        latest_event: event,
    };

    match get_format() {
        OutputFormat::Json => print_issue_detail_json(&detail),
        OutputFormat::Table | OutputFormat::Compact => {
            print_issue_detail(&detail.issue, detail.latest_event.as_ref());
        }
    }

    Ok(())
}
