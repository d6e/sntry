use crate::api::models::ListReleasesParams;
use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_releases_json, print_releases_table};

pub struct ListOptions {
    pub query: Option<String>,
    pub limit: u32,
    pub all: bool,
}

pub async fn list_releases(client: &SentryClient, options: ListOptions) -> Result<()> {
    let params = ListReleasesParams {
        query: options.query,
        limit: Some(options.limit),
        cursor: None,
    };

    let releases = if options.all {
        client.list_all_releases(params).await?
    } else {
        client.list_releases(params).await?
    };

    match get_format() {
        OutputFormat::Json => print_releases_json(&releases),
        OutputFormat::Table | OutputFormat::Compact => print_releases_table(&releases),
    }

    Ok(())
}
