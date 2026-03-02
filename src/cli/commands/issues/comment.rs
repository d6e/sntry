use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_note_json, print_notes_json, print_notes_table, print_success};

pub async fn comment_issue(client: &SentryClient, issue_id: &str, text: &str) -> Result<()> {
    let note = client.create_comment(issue_id, text).await?;

    match get_format() {
        OutputFormat::Json => print_note_json(&note),
        OutputFormat::Table | OutputFormat::Compact => {
            print_success(&format!("Comment {} added to issue {}.", note.id, issue_id));
        }
    }

    Ok(())
}

pub async fn list_issue_comments(client: &SentryClient, issue_id: &str) -> Result<()> {
    let notes = client.list_comments(issue_id).await?;

    match get_format() {
        OutputFormat::Json => print_notes_json(&notes),
        OutputFormat::Table | OutputFormat::Compact => print_notes_table(&notes),
    }

    Ok(())
}
