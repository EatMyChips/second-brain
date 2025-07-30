pub mod tasks_link {
    use crate::{delete_tasks, get_task, get_tasks, post_tasks, put_tasks};
    use crate::backend::props::{Task, NewTask};

    pub async fn new(week: Option<String>, day: Option<String>, id: String) -> i64 {
        post_tasks(NewTask {
            week,
            day,
            container_id: id,
        })
            .await
            .expect("Panic")
            .unwrap()
    }

    pub async fn get(id: i64) -> Task {
        get_task(id).await.expect("Panic").unwrap()
    }

    pub async fn update(id: i64, info: String){
        put_tasks(id, info).await.expect("Panic");
    }

    pub async fn delete(id: i64) {
        delete_tasks(id).await.expect("Panic");
    }

    pub async fn get_all(title: String, week: String, day: Option<String>) -> Vec<i64> {
        let out = get_tasks(title, week, day).await.expect("Panic");
        log::info!("{out:?}");
        out
    }
}