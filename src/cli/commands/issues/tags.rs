use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_tag_details_json, print_tag_details_table};

pub async fn get_issue_tag(client: &SentryClient, issue_id: &str, tag_key: &str) -> Result<()> {
    let tag_details = client.get_issue_tag(issue_id, tag_key).await?;

    match get_format() {
        OutputFormat::Json => print_tag_details_json(&tag_details),
        OutputFormat::Table | OutputFormat::Compact => print_tag_details_table(&tag_details),
    }

    Ok(())
}
