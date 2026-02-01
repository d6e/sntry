use crate::api::models::ListEventsParams;
use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_events_json, print_events_table};

pub struct ListOptions {
    pub issue_id: String,
    pub limit: u32,
    pub all: bool,
}

pub async fn list_events(client: &SentryClient, options: ListOptions) -> Result<()> {
    let params = ListEventsParams {
        limit: Some(options.limit),
        cursor: None,
    };

    let events = if options.all {
        client.list_all_issue_events(&options.issue_id, params).await?
    } else {
        client.list_issue_events(&options.issue_id, params).await?
    };

    match get_format() {
        OutputFormat::Json => print_events_json(&events),
        OutputFormat::Table | OutputFormat::Compact => print_events_table(&events),
    }

    Ok(())
}
