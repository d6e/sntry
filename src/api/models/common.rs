use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Actor {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    #[serde(rename = "type", default)]
    pub actor_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectRef {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub detail: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_actor_with_type() {
        let json = r#"{
            "id": "1",
            "name": "John Doe",
            "email": "john@example.com",
            "type": "user"
        }"#;
        let actor: Actor = serde_json::from_str(json).unwrap();
        assert_eq!(actor.id, "1");
        assert_eq!(actor.name, "John Doe");
        assert_eq!(actor.email.as_deref(), Some("john@example.com"));
        assert_eq!(actor.actor_type.as_deref(), Some("user"));
    }

    #[test]
    fn deserialize_actor_without_type() {
        let json = r#"{
            "id": "1",
            "name": "John Doe",
            "email": "john@example.com"
        }"#;
        let actor: Actor = serde_json::from_str(json).unwrap();
        assert_eq!(actor.id, "1");
        assert_eq!(actor.name, "John Doe");
        assert!(actor.actor_type.is_none());
    }
}
