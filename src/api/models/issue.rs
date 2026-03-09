use super::common::{Actor, ProjectRef};
use super::event::Event;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub short_id: String,
    pub title: String,
    pub status: IssueStatus,
    pub level: String,
    #[serde(default)]
    pub count: String,
    #[serde(default)]
    pub user_count: u64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub permalink: String,
    pub project: ProjectRef,
    pub assigned_to: Option<Actor>,
    #[serde(default)]
    pub is_bookmarked: bool,
    #[serde(default)]
    pub is_subscribed: bool,
    #[serde(default)]
    pub has_seen: bool,
    #[serde(default)]
    pub metadata: IssueMetadata,
    #[serde(default)]
    pub culprit: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IssueStatus {
    Resolved,
    Unresolved,
    Ignored,
    #[serde(rename = "reprocessing")]
    Reprocessing,
}

impl std::fmt::Display for IssueStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueStatus::Resolved => write!(f, "resolved"),
            IssueStatus::Unresolved => write!(f, "unresolved"),
            IssueStatus::Ignored => write!(f, "ignored"),
            IssueStatus::Reprocessing => write!(f, "reprocessing"),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IssueMetadata {
    pub value: Option<String>,
    pub filename: Option<String>,
    pub function: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IssueStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_seen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bookmarked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_window: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<StatusDetails>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_release: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_next_release: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_until_escalating: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteData {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub data: NoteData,
    pub user: Option<Actor>,
    pub date_created: DateTime<Utc>,
}

impl Note {
    pub fn text(&self) -> &str {
        &self.data.text
    }
}

#[derive(Debug, Serialize)]
pub struct CreateNote {
    pub text: String,
}

#[derive(Debug, Default, Clone)]
pub struct ListIssuesParams {
    pub project: Option<Vec<String>>,
    pub query: Option<String>,
    pub status: Option<IssueStatus>,
    pub sort: Option<String>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
    pub environment: Option<Vec<String>>,
    pub stats_period: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryAppInstallation {
    pub uuid: String,
    pub status: String,
    pub app: SentryApp,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryApp {
    pub slug: String,
    #[serde(default)]
    pub name: Option<String>,
    pub uuid: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExternalIssue {
    pub issue_id: u64,
    pub web_url: String,
    pub project: String,
    pub identifier: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIssue {
    pub id: String,
    pub issue_id: String,
    pub service_type: String,
    pub display_name: String,
    pub web_url: String,
}

/// Combined issue + latest event for detailed JSON output.
/// Uses flatten so all Issue fields remain at the top level.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueDetail {
    #[serde(flatten)]
    pub issue: Issue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_event: Option<Event>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagDetails {
    pub key: String,
    pub total_values: u64,
    #[serde(default)]
    pub top_values: Vec<TagValue>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagValue {
    pub value: String,
    pub count: u64,
    #[serde(default)]
    pub first_seen: Option<String>,
    #[serde(default)]
    pub last_seen: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_note_from_api_response() {
        let json = r#"{
            "id": "12345",
            "type": "note",
            "data": {"text": "This is a comment"},
            "user": {
                "id": "1",
                "name": "John Doe",
                "email": "john@example.com"
            },
            "dateCreated": "2023-01-01T00:00:00Z"
        }"#;
        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note.id, "12345");
        assert_eq!(note.text(), "This is a comment");
        let user = note.user.unwrap();
        assert_eq!(user.name, "John Doe");
        assert!(user.actor_type.is_none());
    }

    #[test]
    fn deserialize_tag_details_full() {
        let json = r#"{
            "key": "environment",
            "totalValues": 42,
            "topValues": [
                {
                    "value": "production",
                    "count": 30,
                    "firstSeen": "2024-01-01T00:00:00Z",
                    "lastSeen": "2024-06-01T00:00:00Z"
                },
                {
                    "value": "staging",
                    "count": 12,
                    "firstSeen": "2024-02-01T00:00:00Z",
                    "lastSeen": "2024-05-15T00:00:00Z"
                }
            ]
        }"#;
        let tag: TagDetails = serde_json::from_str(json).unwrap();
        assert_eq!(tag.key, "environment");
        assert_eq!(tag.total_values, 42);
        assert_eq!(tag.top_values.len(), 2);
        assert_eq!(tag.top_values[0].value, "production");
        assert_eq!(tag.top_values[0].count, 30);
        assert_eq!(
            tag.top_values[0].first_seen.as_deref(),
            Some("2024-01-01T00:00:00Z")
        );
        assert_eq!(tag.top_values[1].value, "staging");
        assert_eq!(tag.top_values[1].count, 12);
    }

    #[test]
    fn deserialize_tag_details_empty_top_values() {
        let json = r#"{
            "key": "custom_tag",
            "totalValues": 0,
            "topValues": []
        }"#;
        let tag: TagDetails = serde_json::from_str(json).unwrap();
        assert_eq!(tag.key, "custom_tag");
        assert_eq!(tag.total_values, 0);
        assert!(tag.top_values.is_empty());
    }

    #[test]
    fn deserialize_note_with_null_user() {
        let json = r#"{
            "id": "12345",
            "data": {"text": "automated comment"},
            "user": null,
            "dateCreated": "2023-01-01T00:00:00Z"
        }"#;
        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note.text(), "automated comment");
        assert!(note.user.is_none());
    }

    fn make_test_issue() -> Issue {
        let json = r#"{
            "id": "123",
            "shortId": "PROJ-123",
            "title": "Test Issue",
            "status": "unresolved",
            "level": "error",
            "count": "5",
            "userCount": 3,
            "firstSeen": "2023-01-01T00:00:00Z",
            "lastSeen": "2023-06-01T00:00:00Z",
            "permalink": "https://sentry.io/issues/123/",
            "project": {"id": "1", "name": "test-project", "slug": "test-project"},
            "assignedTo": null,
            "isBookmarked": false,
            "isSubscribed": false,
            "hasSeen": false,
            "metadata": {},
            "culprit": null
        }"#;
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn issue_detail_flattens_issue_fields() {
        let detail = IssueDetail {
            issue: make_test_issue(),
            latest_event: None,
        };
        let json_str = serde_json::to_string(&detail).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Issue fields should be at the top level, not nested under "issue"
        assert_eq!(value["id"], "123");
        assert_eq!(value["shortId"], "PROJ-123");
        assert_eq!(value["title"], "Test Issue");
        assert!(value.get("issue").is_none());
    }

    #[test]
    fn issue_detail_omits_latest_event_when_none() {
        let detail = IssueDetail {
            issue: make_test_issue(),
            latest_event: None,
        };
        let json_str = serde_json::to_string(&detail).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert!(value.get("latestEvent").is_none());
    }

    #[test]
    fn issue_detail_includes_latest_event_when_some() {
        use std::collections::HashMap;

        let event = Event {
            id: "evt-1".to_string(),
            message: Some("test".to_string()),
            entries: vec![],
            tags: vec![],
            contexts: HashMap::new(),
            breadcrumbs: None,
            user: None,
        };
        let detail = IssueDetail {
            issue: make_test_issue(),
            latest_event: Some(event),
        };
        let json_str = serde_json::to_string(&detail).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert!(value.get("latestEvent").is_some());
        assert_eq!(value["latestEvent"]["id"], "evt-1");
        assert_eq!(value["latestEvent"]["message"], "test");
        // Issue fields should still be at the top level
        assert_eq!(value["id"], "123");
    }
}
