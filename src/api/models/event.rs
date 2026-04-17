use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub entries: Vec<EventEntry>,
    #[serde(default)]
    pub tags: Vec<EventTag>,
    #[serde(default)]
    pub contexts: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub breadcrumbs: Option<BreadcrumbsData>,
    #[serde(default)]
    pub user: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum EventEntry {
    Exception {
        data: ExceptionData,
    },
    Message {
        data: MessageData,
    },
    Breadcrumbs {
        data: BreadcrumbsData,
    },
    #[serde(rename = "debugmeta")]
    DebugMeta {
        data: serde_json::Value,
    },
    Threads {
        data: serde_json::Value,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExceptionData {
    pub values: Vec<ExceptionValue>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionValue {
    #[serde(rename = "type")]
    pub exception_type: String,
    pub value: String,
    #[serde(default)]
    pub stacktrace: Option<Stacktrace>,
    #[serde(default)]
    pub mechanism: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stacktrace {
    pub frames: Vec<StackFrame>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackFrame {
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub function: Option<String>,
    #[serde(default)]
    pub lineno: Option<u32>,
    #[serde(default)]
    pub colno: Option<u32>,
    #[serde(default)]
    pub context_line: Option<String>,
    #[serde(default)]
    pub pre_context: Vec<String>,
    #[serde(default)]
    pub post_context: Vec<String>,
    #[serde(default)]
    pub in_app: bool,
    #[serde(default)]
    pub instruction_addr: Option<String>,
    #[serde(default)]
    pub package: Option<String>,
    #[serde(default)]
    pub trust: Option<String>,
    #[serde(default)]
    pub image_addr: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageData {
    pub formatted: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BreadcrumbsData {
    pub values: Vec<Breadcrumb>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Breadcrumb {
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(rename = "type", default)]
    pub breadcrumb_type: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventTag {
    pub key: String,
    pub value: String,
}

/// Lighter event struct for list responses
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventListItem {
    pub id: String,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    pub date_created: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub tags: Vec<EventTag>,
    #[serde(rename = "eventType", default)]
    pub event_type: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListEventsParams {
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: String,
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub name: String,
    #[serde(default)]
    pub mimetype: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub date_created: Option<String>,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
}
