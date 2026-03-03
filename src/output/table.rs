use crate::api::models::{Breadcrumb, Event, EventEntry, EventListItem, ExceptionValue, Issue, Note, Release, StackFrame};
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

pub fn print_issue_detail(issue: &Issue, event: Option<&Event>) {
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
            assigned.email.as_deref().unwrap_or(assigned.actor_type.as_deref().unwrap_or("unknown"))
        );
    } else {
        println!("{:<12} {}", "Assigned:".bold(), "Unassigned".dimmed());
    }

    if let Some(ref culprit) = issue.culprit {
        println!("{:<12} {}", "Culprit:".bold(), culprit);
    }

    println!();
    println!("{:<12} {}", "Link:".bold(), issue.permalink.blue());

    // Print event details if available
    if let Some(event) = event {
        print_event_details(event);
    }

    println!();
}

pub fn print_event_details(event: &Event) {
    let separator = "=".repeat(80);

    println!();
    println!("{separator}");
    println!("{}", "Event Details".bold().cyan());
    println!("{separator}");

    // Print exception information
    for entry in &event.entries {
        match entry {
            EventEntry::Exception { data } => {
                for (i, exception) in data.values.iter().enumerate() {
                    if i > 0 {
                        println!("\n{}", "Caused by:".yellow().bold());
                    }
                    print_exception(exception);
                }
            }
            EventEntry::Message { data } => {
                println!("\n{}:", "Message".bold());
                println!("{}", data.formatted);
            }
            _ => {}
        }
    }

    // Print breadcrumbs
    let breadcrumbs = event.breadcrumbs.as_ref()
        .or_else(|| {
            event.entries.iter().find_map(|entry| match entry {
                EventEntry::Breadcrumbs { data } => Some(data),
                _ => None,
            })
        });

    if let Some(breadcrumbs_data) = breadcrumbs {
        print_breadcrumbs(&breadcrumbs_data.values);
    }

    // Print tags
    if !event.tags.is_empty() {
        println!("\n{}:", "Tags".bold());
        for tag in &event.tags {
            println!("  {} = {}", tag.key.cyan(), tag.value);
        }
    }

    // Print context (showing important ones)
    if !event.contexts.is_empty() {
        println!("\n{}:", "Context".bold());
        for (key, value) in &event.contexts {
            if let Some(obj) = value.as_object() {
                println!("  {}:", key.cyan());
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        println!("    {} = {}", k, s);
                    } else if let Some(n) = v.as_number() {
                        println!("    {} = {}", k, n);
                    } else if let Some(b) = v.as_bool() {
                        println!("    {} = {}", k, b);
                    }
                }
            }
        }
    }
}

fn print_exception(exception: &ExceptionValue) {
    println!("\n{}:", "Exception".bold().red());
    println!("  {}: {}", "Type".bold(), exception.exception_type.red());
    println!("  {}: {}", "Value".bold(), exception.value);

    if let Some(stacktrace) = &exception.stacktrace {
        println!("\n  {}:", "Stack Trace".bold());

        // Print frames in reverse order (most recent first)
        for frame in stacktrace.frames.iter().rev() {
            print_stack_frame(frame);
        }
    }
}

fn print_stack_frame(frame: &StackFrame) {
    let in_app_marker = if frame.in_app { "▶ ".green() } else { "  ".into() };

    let function = frame.function.as_deref().unwrap_or("<unknown>");
    let filename = frame.filename.as_deref().unwrap_or("<unknown>");

    if let Some(lineno) = frame.lineno {
        println!(
            "{}  {} in {}:{}",
            in_app_marker,
            function.cyan(),
            filename.dimmed(),
            lineno
        );
    } else {
        println!(
            "{}  {} in {}",
            in_app_marker,
            function.cyan(),
            filename.dimmed()
        );
    }

    // Print context if available
    if let Some(context_line) = &frame.context_line {
        if let Some(lineno) = frame.lineno {
            println!("      {} {}", format!("{:>4}", lineno).dimmed(), context_line);
        }
    }
}

fn print_breadcrumbs(breadcrumbs: &[Breadcrumb]) {
    if breadcrumbs.is_empty() {
        return;
    }

    println!("\n{}:", "Breadcrumbs".bold());

    // Show last 10 breadcrumbs
    let start = breadcrumbs.len().saturating_sub(10);
    for breadcrumb in &breadcrumbs[start..] {
        let timestamp = breadcrumb.timestamp.as_deref().unwrap_or("");
        let category = breadcrumb.category.as_deref().unwrap_or("default");
        let level = breadcrumb.level.as_deref().unwrap_or("info");
        let message = breadcrumb.message.as_deref().unwrap_or("");

        let level_color = match level {
            "error" => message.red(),
            "warning" => message.yellow(),
            _ => message.normal(),
        };

        println!(
            "  {} [{}] {}: {}",
            timestamp.dimmed(),
            category.cyan(),
            level,
            level_color
        );
    }
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

#[derive(Tabled)]
struct EventRow {
    #[tabled(rename = "Event ID")]
    event_id: String,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Platform")]
    platform: String,
    #[tabled(rename = "Created")]
    created: String,
}

impl From<&EventListItem> for EventRow {
    fn from(event: &EventListItem) -> Self {
        let created = chrono::DateTime::parse_from_rfc3339(&event.date_created)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();

        Self {
            event_id: event.event_id.clone().unwrap_or_else(|| event.id.clone()),
            title: truncate_string(
                event.title.as_deref()
                    .or(event.message.as_deref())
                    .unwrap_or("<no title>"),
                50
            ),
            platform: event.platform.clone().unwrap_or_else(|| "-".to_string()),
            created: created.map(|dt| format_relative_time(&dt)).unwrap_or_else(|| "-".to_string()),
        }
    }
}

pub fn print_events_table(events: &[EventListItem]) {
    if events.is_empty() {
        println!("No events found.");
        return;
    }

    let rows: Vec<EventRow> = events.iter().map(EventRow::from).collect();
    let table = Table::new(rows).with(Style::rounded()).to_string();

    println!("{table}");
    println!("Showing {} event(s)", events.len());
}

#[derive(Tabled)]
struct NoteRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Author")]
    author: String,
    #[tabled(rename = "Text")]
    text: String,
    #[tabled(rename = "Date")]
    date: String,
}

impl From<&Note> for NoteRow {
    fn from(note: &Note) -> Self {
        let author = note
            .user
            .as_ref()
            .map(|u| u.name.clone())
            .unwrap_or_else(|| "-".to_string());

        Self {
            id: note.id.clone(),
            author,
            text: truncate_string(note.text(), 60),
            date: format_relative_time(&note.date_created),
        }
    }
}

pub fn print_notes_table(notes: &[Note]) {
    if notes.is_empty() {
        println!("No comments found.");
        return;
    }

    let rows: Vec<NoteRow> = notes.iter().map(NoteRow::from).collect();
    let table = Table::new(rows).with(Style::rounded()).to_string();

    println!("{table}");
    println!("Showing {} comment(s)", notes.len());
}
