use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_event_details, print_event_json};

pub async fn view_event(client: &SentryClient, event_id: &str, project: &str) -> Result<()> {
    let event = client.get_event(project, event_id).await?;

    match get_format() {
        OutputFormat::Json => print_event_json(&event),
        OutputFormat::Table | OutputFormat::Compact => print_event_details(&event),
    }

    Ok(())
}
