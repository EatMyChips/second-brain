use serde::{Deserialize, Serialize};
use crate::{delete_tasks, get_task, get_tasks, post_tasks, put_tasks};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct TaskInput {
    pub title: String,
    pub info: String,
    pub week: Option<String>,
    pub day: Option<String>,
    pub container_id: String,
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

impl Task{
    pub async fn new(input: TaskInput) -> Task {
        //TODO:: remove the unwraps and replace with error handling

        Task::get_with_id(post_tasks(input).await.expect("Panic").unwrap()).await
    }

    async fn get_with_id(id: i32) -> Task {
        get_task(id).await.expect("Panic").unwrap()
    }

    pub async fn update(mut self, title: String, info: String) {
        self.title = title;
        self.info = info;
        put_tasks(self).await.expect("Panic");
    }

    pub async fn delete(&self) {
        delete_tasks(self.id).await.expect("Panic");
    }

    pub async fn get_all(title: String, week: String) -> Vec<Self>{
        get_tasks(title, week).await.expect("Panic")
    }
}