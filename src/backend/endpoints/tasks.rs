use dioxus::prelude::*;

#[cfg(feature = "server")]
use rusqlite::{params, Connection, ToSql, Result as SqlResult};
#[cfg(feature = "server")]
use super::super::init_database::DB;
use super::super::props::{Task, TaskInput};


#[server]
pub async fn post_tasks(container_title: String, task: TaskInput) -> Result<Option<Task>, ServerFnError> {
    DB.with(|f| {
        // First, look up the container ID
        let mut stmt = f.prepare("SELECT id FROM containers WHERE title = ?1")?;
        let mut rows = stmt.query(params![container_title])?;

        let container_id = if let Some(row) = rows.next()? {
            row.get::<_, i32>(0)?
        } else {
            return Err(ServerFnError::ServerError("Container not found".into()));
        };

        f.execute(
            "INSERT INTO tasks (title, info, weeks, days, container_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task.title,
                task.info,
                task.week.as_deref(),
                task.day.as_deref(),
                container_id,
            ],
        )?;

        let mut stmt = f.prepare(
            "SELECT id, title, info, weeks, days, container_id FROM tasks WHERE id = ?1"
        )?;
        let mut rows = stmt.query((f.last_insert_rowid(),))?;


        if let Some(row) = rows.next()? {
            let task = Task {
                id: row.get(0)?,
                title: row.get(1)?,
                info: row.get(2)?,
                week: row.get(3)?,
                day: row.get(4)?,
                container_id: row.get(5)?,
            };
            Ok(Some(task))
        } else {
            Ok(None)
        }
    })
}

#[server]
pub async fn put_tasks(id: i32, task: TaskInput) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "UPDATE tasks SET title = ?1, info = ?2, weeks = ?3, days = ?4, container_id = ?5 WHERE id = ?6",
            params![
                task.title,
                task.info,
                task.week.as_deref(),
                task.day.as_deref(),
                task.container_id,
                id,
            ],
        )?;
        Ok(())
    })
}

#[server]
pub async fn delete_tasks(id: i32) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    })
}

#[server]
pub async fn get_tasks(container_title: String, current_week: String) -> Result<Vec<Task>, ServerFnError> {
    DB.with(|f| {
        // Use JOIN to fetch tasks based on the container title
        let mut stmt = f.prepare(
            "SELECT t.id, t.title, t.info, t.weeks, t.days, t.container_id
             FROM tasks t
             JOIN containers c ON t.container_id = c.id
             WHERE c.title = ?1 AND t.weeks = ?2"
        )?;
        let mut rows = stmt.query(params![container_title, current_week])?;

        let mut tasks = Vec::new();

        while let Some(row) = rows.next()? {
            let task = Task {
                id: row.get(0)?,
                title: row.get(1)?,
                info: row.get(2)?,
                week: row.get(3)?,
                day: row.get(4)?,
                container_id: row.get(5)?,
            };
            tasks.push(task);
        }

        Ok(tasks)
    })
}