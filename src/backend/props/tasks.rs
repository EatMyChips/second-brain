use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct TaskInput {
    pub title: String,
    pub info: String,
    pub week: Option<String>,
    pub day: Option<String>,
    pub container_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub info: String,
    pub week: Option<String>,
    pub day: Option<String>,
    pub container_id: i32,
}