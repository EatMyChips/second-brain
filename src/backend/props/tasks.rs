use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTask {
    pub week: Option<String>,
    pub day: Option<String>,
    pub container_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub info: String,
    pub completed: bool,
    pub week: Option<String>,
    pub day: Option<String>,
    pub container_id: i64,
}

impl Task {

}
