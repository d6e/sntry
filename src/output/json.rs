use crate::api::models::{Issue, Release};

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
