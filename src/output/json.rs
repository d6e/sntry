use crate::api::models::{Event, EventListItem, ExternalIssue, Issue, Note, Release};

pub fn print_issues_json(issues: &[Issue]) {
    let json = serde_json::to_string_pretty(issues).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_issue_json(issue: &Issue) {
    let json = serde_json::to_string_pretty(issue).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

pub fn print_releases_json(releases: &[Release]) {
    let json = serde_json::to_string_pretty(releases).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_release_details_json(release: &Release) {
    let json = serde_json::to_string_pretty(release).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

pub fn print_events_json(events: &[EventListItem]) {
    let json = serde_json::to_string_pretty(events).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_event_json(event: &Event) {
    let json = serde_json::to_string_pretty(event).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

pub fn print_notes_json(notes: &[Note]) {
    let json = serde_json::to_string_pretty(notes).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_note_json(note: &Note) {
    let json = serde_json::to_string_pretty(note).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

pub fn print_external_issue_json(issue: &ExternalIssue) {
    let json = serde_json::to_string_pretty(issue).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

