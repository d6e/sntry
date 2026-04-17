use std::path::Path;

use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_success};

pub async fn list_attachments(client: &SentryClient, event_id: &str, project: &str) -> Result<()> {
    let attachments = client.list_event_attachments(project, event_id).await?;

    match get_format() {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string(&attachments).unwrap_or_else(|_| "[]".to_string()));
        }
        OutputFormat::Table | OutputFormat::Compact => {
            if attachments.is_empty() {
                println!("No attachments found for event {}", event_id);
                return Ok(());
            }
            println!(
                "{:<12} {:<30} {:<20} {:<10}",
                "ID", "Name", "Type", "Size"
            );
            println!("{}", "-".repeat(75));
            for att in &attachments {
                let size = att
                    .size
                    .map(|s| format_size(s))
                    .unwrap_or_else(|| "?".to_string());
                println!(
                    "{:<12} {:<30} {:<20} {:<10}",
                    att.id, att.name, att.attachment_type, size
                );
            }
        }
    }

    Ok(())
}

pub async fn download_attachment(
    client: &SentryClient,
    event_id: &str,
    project: &str,
    output: &str,
    attachment_id: Option<&str>,
) -> Result<()> {
    let attachments = client.list_event_attachments(project, event_id).await?;

    if attachments.is_empty() {
        return Err(crate::error::SentryCliError::NotFound(
            "No attachments found for this event".to_string(),
        ));
    }

    let attachment = if let Some(id) = attachment_id {
        attachments
            .iter()
            .find(|a| a.id == id)
            .ok_or_else(|| {
                crate::error::SentryCliError::NotFound(format!("Attachment {} not found", id))
            })?
    } else {
        // Default: download the first minidump, or first attachment
        attachments
            .iter()
            .find(|a| {
                a.attachment_type == "event.minidump"
                    || a.name.ends_with(".dmp")
                    || a.name.ends_with(".mdmp")
            })
            .unwrap_or(&attachments[0])
    };

    let data = client
        .download_attachment(project, event_id, &attachment.id)
        .await?;

    let safe_name = Path::new(&attachment.name)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| format!("attachment-{}", attachment.id));

    let output_path = Path::new(output);
    let final_path = if output_path.is_dir() || output.ends_with('/') {
        std::fs::create_dir_all(output_path)?;
        output_path.join(&safe_name)
    } else {
        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        output_path.to_path_buf()
    };

    std::fs::write(&final_path, &data)?;
    print_success(&format!(
        "Downloaded {} ({}) to {}",
        safe_name,
        format_size(data.len() as u64),
        final_path.display()
    ));

    Ok(())
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
