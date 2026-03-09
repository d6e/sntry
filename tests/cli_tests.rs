use assert_cmd::Command;
use predicates::prelude::*;

fn sentry_cli() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("sentry"))
}

#[test]
fn test_help() {
    sentry_cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CLI tool for managing Sentry issues",
        ));
}

#[test]
fn test_version() {
    sentry_cli()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("sentry"));
}

#[test]
fn test_issues_help() {
    sentry_cli()
        .args(["issues", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage Sentry issues"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("view"))
        .stdout(predicate::str::contains("resolve"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("comment"))
        .stdout(predicate::str::contains("link"))
        .stdout(predicate::str::contains("tags"));
}

#[test]
fn test_issues_list_help() {
    sentry_cli()
        .args(["issues", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--project"))
        .stdout(predicate::str::contains("--status"))
        .stdout(predicate::str::contains("--format"))
        .stdout(predicate::str::contains("--all"))
        .stdout(predicate::str::contains("--environment"))
        .stdout(predicate::str::contains("--period"))
        .stdout(predicate::str::contains("--start"))
        .stdout(predicate::str::contains("--end"));
}

#[test]
fn test_issues_list_period_conflicts_with_start() {
    sentry_cli()
        .args([
            "issues",
            "list",
            "--period",
            "24h",
            "--start",
            "2024-01-01T00:00:00Z",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_issues_list_end_requires_start() {
    sentry_cli()
        .args(["issues", "list", "--end", "2024-01-31T00:00:00Z"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_issues_list_sort_rejects_invalid() {
    sentry_cli()
        .args(["issues", "list", "--sort", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn test_config_help() {
    sentry_cli()
        .args(["config", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("set"));
}

#[test]
fn test_config_show() {
    sentry_cli()
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Config file:"));
}

#[test]
fn test_issues_list_requires_auth() {
    // Without auth, should fail with an auth-related error
    sentry_cli()
        .args(["--org", "test-org", "issues", "list"])
        .env_remove("SENTRY_AUTH_TOKEN")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("No auth token found")
                .or(predicate::str::contains("Permission denied")),
        );
}

#[test]
fn test_issues_list_requires_org() {
    // Without org, should fail with org or auth error
    sentry_cli()
        .args(["--token", "fake-token", "issues", "list"])
        .env_remove("SENTRY_ORG")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("No organization specified")
                .or(predicate::str::contains("Authentication failed")),
        );
}

#[test]
fn test_issues_comment_help() {
    sentry_cli()
        .args(["issues", "comment", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("comment"))
        .stdout(predicate::str::contains("--list"));
}

#[test]
fn test_issues_link_help() {
    sentry_cli()
        .args(["issues", "link", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--url"))
        .stdout(predicate::str::contains("--project"))
        .stdout(predicate::str::contains("--identifier"))
        .stdout(predicate::str::contains("--integration"));
}

#[test]
fn test_issues_tags_help() {
    sentry_cli()
        .args(["issues", "tags", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ISSUE_ID"))
        .stdout(predicate::str::contains("TAG_KEY"));
}

#[test]
fn test_output_format_json() {
    sentry_cli()
        .args(["issues", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("json"))
        .stdout(predicate::str::contains("table"));
}
