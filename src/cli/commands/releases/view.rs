use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_release_details_json, print_release_details};

pub async fn view_release(client: &SentryClient, version: &str) -> Result<()> {
    let release = client.get_release(version).await?;

    match get_format() {
        OutputFormat::Json => print_release_details_json(&release),
        OutputFormat::Table | OutputFormat::Compact => print_release_details(&release),
    }

    Ok(())
}
