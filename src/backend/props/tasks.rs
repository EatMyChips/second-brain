use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskPayload {
    pub week: Option<String>,
    pub day: Option<String>,
    pub container_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskResponse {
    pub id: i64,
    pub title: String,
    pub info: String,
    pub completed: bool,
}
