use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_external_issues_json, print_external_issues_table};

pub async fn list_external_issues(client: &SentryClient, issue_id: &str) -> Result<()> {
    let links = client.list_external_issues(issue_id).await?;

    match get_format() {
        OutputFormat::Json => print_external_issues_json(&links),
        OutputFormat::Table | OutputFormat::Compact => print_external_issues_table(&links),
    }

    Ok(())
}
