use crate::api::models::{IssueStatus, ListIssuesParams};
use crate::api::SentryClient;
use crate::cli::args::OutputFormat;
use crate::error::Result;
use crate::output::{get_format, print_issues_json, print_issues_table};

pub struct ListOptions {
    pub project: Option<String>,
    pub status: Option<String>,
    pub query: Option<String>,
    pub sort: String,
    pub limit: u32,
    pub all: bool,
    pub environment: Option<String>,
    pub period: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

pub async fn list_issues(client: &SentryClient, options: ListOptions) -> Result<()> {
    let status_filter = options
        .status
        .as_ref()
        .and_then(|s| match s.to_lowercase().as_str() {
            "resolved" => Some(IssueStatus::Resolved),
            "unresolved" => Some(IssueStatus::Unresolved),
            "ignored" => Some(IssueStatus::Ignored),
            _ => None,
        });

    let projects = options
        .project
        .map(|p| p.split(',').map(|s| s.trim().to_string()).collect());

    let environments = options
        .environment
        .map(|e| e.split(',').map(|s| s.trim().to_string()).collect());

    let params = ListIssuesParams {
        project: projects,
        query: options.query,
        status: status_filter,
        sort: Some(options.sort),
        limit: Some(options.limit),
        cursor: None,
        environment: environments,
        stats_period: options.period,
        start: options.start,
        end: options.end,
    };

    let issues = if options.all {
        client.list_all_issues(params).await?
    } else {
        client.list_issues(params).await?
    };

    match get_format() {
        OutputFormat::Json => print_issues_json(&issues),
        OutputFormat::Table | OutputFormat::Compact => print_issues_table(&issues),
    }

    Ok(())
}
