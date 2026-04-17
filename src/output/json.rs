use crate::api::models::{Event, EventListItem, ExternalIssue, Issue, IssueDetail, Note, Release, TagDetails};

pub fn print_issues_json(issues: &[Issue]) {
    let json = serde_json::to_string(issues).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_releases_json(releases: &[Release]) {
    let json = serde_json::to_string(releases).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_release_details_json(release: &Release) {
    let json = serde_json::to_string(release).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

pub fn print_events_json(events: &[EventListItem]) {
    let json = serde_json::to_string(events).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_event_json(event: &Event) {
    let json = serde_json::to_string(event).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

pub fn print_notes_json(notes: &[Note]) {
    let json = serde_json::to_string(notes).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_note_json(note: &Note) {
    let json = serde_json::to_string(note).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

pub fn print_external_issue_json(issue: &ExternalIssue) {
    let json = serde_json::to_string(issue).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

pub fn print_external_issues_json(issues: &[ExternalIssue]) {
    let json = serde_json::to_string(issues).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_issue_detail_json(detail: &IssueDetail) {
    let json = serde_json::to_string(detail).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

pub fn print_tag_details_json(tag_details: &TagDetails) {
    let json = serde_json::to_string(tag_details).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);
}

