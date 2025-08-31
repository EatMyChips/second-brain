pub mod tasks_link {
    use crate::{delete_tasks, get_tasks, post_tasks, put_tasks, put_completed, get_task, TaskResponse, TaskPayload};

    pub async fn new(week: Option<String>, day: Option<String>, id: String) -> TaskResponse {
        let id = post_tasks(TaskPayload {
            week,
            day,
            container_id: id,
        })
            .await
            .expect("Panic")
            .unwrap();

        get_task(id)
            .await
            .expect("Panic")
            .unwrap()
    }

    pub async fn update(id: i64, info: String){
        put_tasks(id, info).await.expect("Panic");
    }

    pub async fn delete(id: i64) {
        delete_tasks(id).await.expect("Panic");
    }

    pub async fn get_all(title: String, week: String, day: Option<String>) -> Vec<TaskResponse> {
        get_tasks(title, week, day).await.expect("Panic")
    }

    pub async fn update_completed(id: i64, completed: bool) {
        put_completed(id, completed).await.expect("Panic");
    }
}

pub mod checks_link {

}