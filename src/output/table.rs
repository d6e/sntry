use crate::api::models::{Issue, Release};
use chrono::{DateTime, Utc};
use colored::Colorize;
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct IssueRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Short ID")]
    short_id: String,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Events")]
    events: String,
    #[tabled(rename = "Last Seen")]
    last_seen: String,
}

impl From<&Issue> for IssueRow {
    fn from(issue: &Issue) -> Self {
        Self {
            id: issue.id.clone(),
            short_id: issue.short_id.clone(),
            title: truncate_string(&issue.title, 50),
            status: format_status_colored(&issue.status),
            events: issue.count.clone(),
            last_seen: format_relative_time(&issue.last_seen),
        }
    }
}

pub fn print_issues_table(issues: &[Issue]) {
    if issues.is_empty() {
        println!("No issues found.");
        return;
    }

    let rows: Vec<IssueRow> = issues.iter().map(IssueRow::from).collect();
    let table = Table::new(rows).with(Style::rounded()).to_string();

    println!("{table}");
    println!("Showing {} issue(s)", issues.len());
}

pub fn print_issue_detail(issue: &Issue) {
    let separator = "=".repeat(80);

    println!();
    println!("{}: {}", "Issue".bold(), issue.short_id.cyan());
    println!("{separator}");
    println!("{:<12} {}", "Title:".bold(), issue.title);
    println!("{:<12} {}", "Status:".bold(), format_status(&issue.status));
    println!("{:<12} {}", "Level:".bold(), issue.level);
    println!(
        "{:<12} {} ({})",
        "Project:".bold(),
        issue.project.name,
        issue.project.slug
    );
    println!(
        "{:<12} {}",
        "First Seen:".bold(),
        issue.first_seen.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "{:<12} {}",
        "Last Seen:".bold(),
        issue.last_seen.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!();
    println!(
        "{:<12} {} total ({} users affected)",
        "Events:".bold(),
        issue.count,
        issue.user_count
    );
    println!();

    if let Some(ref assigned) = issue.assigned_to {
        println!(
            "{:<12} {} ({})",
            "Assigned:".bold(),
            assigned.name,
            assigned.email.as_deref().unwrap_or(&assigned.actor_type)
        );
    } else {
        println!("{:<12} {}", "Assigned:".bold(), "Unassigned".dimmed());
    }

    if let Some(ref culprit) = issue.culprit {
        println!("{:<12} {}", "Culprit:".bold(), culprit);
    }

    println!();
    println!("{:<12} {}", "Link:".bold(), issue.permalink.blue());
    println!();
}

fn format_status(status: &crate::api::models::IssueStatus) -> String {
    match status {
        crate::api::models::IssueStatus::Resolved => "Resolved".green().to_string(),
        crate::api::models::IssueStatus::Unresolved => "Unresolved".red().to_string(),
        crate::api::models::IssueStatus::Ignored => "Ignored".yellow().to_string(),
        crate::api::models::IssueStatus::Reprocessing => "Reprocessing".cyan().to_string(),
    }
}

fn format_status_colored(status: &crate::api::models::IssueStatus) -> String {
    match status {
        crate::api::models::IssueStatus::Resolved => "resolved".green().to_string(),
        crate::api::models::IssueStatus::Unresolved => "unresolved".red().to_string(),
        crate::api::models::IssueStatus::Ignored => "ignored".yellow().to_string(),
        crate::api::models::IssueStatus::Reprocessing => "reprocessing".cyan().to_string(),
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len - 3).collect();
        format!("{}...", truncated)
    }
}

fn format_relative_time(dt: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*dt);

    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{} min ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} hr ago", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{} days ago", duration.num_days())
    } else {
        dt.format("%Y-%m-%d").to_string()
    }
}

pub fn print_success(message: &str) {
    if super::is_quiet() {
        return;
    }
    println!("{} {}", "✓".green(), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

#[derive(Tabled)]
struct ReleaseRow {
    #[tabled(rename = "Version")]
    version: String,
    #[tabled(rename = "Created")]
    created: String,
    #[tabled(rename = "Projects")]
    projects: String,
    #[tabled(rename = "New Issues")]
    new_groups: String,
}

impl From<&Release> for ReleaseRow {
    fn from(release: &Release) -> Self {
        let created = chrono::DateTime::parse_from_rfc3339(&release.date_created)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();

        let projects_str = if release.projects.is_empty() {
            "-".to_string()
        } else if release.projects.len() == 1 {
            release.projects[0].slug.clone()
        } else {
            format!("{} projects", release.projects.len())
        };

        Self {
            version: release.version.clone(),
            created: created.map(|dt| format_relative_time(&dt)).unwrap_or_else(|| "-".to_string()),
            projects: projects_str,
            new_groups: release.new_groups.to_string(),
        }
    }
}

pub fn print_releases_table(releases: &[Release]) {
    if releases.is_empty() {
        println!("No releases found.");
        return;
    }

    let rows: Vec<ReleaseRow> = releases.iter().map(ReleaseRow::from).collect();
    let table = Table::new(rows).with(Style::rounded()).to_string();

    println!("{table}");
    println!("Showing {} release(s)", releases.len());
}

pub fn print_release_details(release: &Release) {
    let separator = "=".repeat(80);

    println!();
    println!("{}: {}", "Release".bold(), release.version.cyan());
    println!("{separator}");

    let created = chrono::DateTime::parse_from_rfc3339(&release.date_created)
        .map(|dt| dt.with_timezone(&Utc))
        .ok();

    if let Some(dt) = created {
        println!("{:<15} {}", "Created:".bold(), dt.format("%Y-%m-%d %H:%M:%S UTC"));
    }

    if let Some(ref date_released) = release.date_released {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_released) {
            println!("{:<15} {}", "Released:".bold(), dt.with_timezone(&Utc).format("%Y-%m-%d %H:%M:%S UTC"));
        }
    }

    println!("{:<15} {}", "New Issues:".bold(), release.new_groups);

    if !release.projects.is_empty() {
        println!("\n{}:", "Projects".bold());
        for project in &release.projects {
            println!("  • {} ({})", project.name, project.slug);
        }
    }

    if !release.authors.is_empty() {
        println!("\n{}:", "Authors".bold());
        for author in &release.authors {
            if let Some(ref email) = author.email {
                println!("  • {} <{}>", author.name, email);
            } else {
                println!("  • {}", author.name);
            }
        }
    }

    if let Some(ref last_deploy) = release.last_deploy {
        println!("\n{}: {}", "Last Deploy".bold(), last_deploy.environment.green());
    }

    println!();
}
