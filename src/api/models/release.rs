use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub version: String,
    pub short_version: Option<String>,
    pub date_created: String,
    pub date_released: Option<String>,
    pub new_groups: u32,
    pub last_event: Option<String>,
    pub first_event: Option<String>,
    pub last_deploy: Option<DeployInfo>,
    pub authors: Vec<Author>,
    pub projects: Vec<ProjectRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployInfo {
    pub id: String,
    pub environment: String,
    #[serde(rename = "dateFinished")]
    pub date_finished: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRef {
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Clone, Default)]
pub struct ListReleasesParams {
    pub query: Option<String>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}
