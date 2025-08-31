use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::super::init_database::DB;
use super::super::props::{TaskPayload, TaskResponse};
#[cfg(feature = "server")]
use rusqlite::{params, Connection, Result as SqlResult, ToSql};

#[server]
pub async fn post_tasks(task: TaskPayload) -> Result<Option<i64>, ServerFnError> {
    let empty_string: String = String::new();
    DB.with(|f| {
        // First, look up the container ID
        let mut stmt = f.prepare("SELECT id FROM containers WHERE title = ?1")?;
        let mut rows = stmt.query(params![task.container_id])?;

        let container_id = if let Some(row) = rows.next()? {
            row.get::<_, i32>(0)?
        } else {
            return Err(ServerFnError::ServerError("Container not found".into()));
        };

        f.execute(
            "INSERT INTO todo (title, info, weeks, days, completed, container_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                empty_string,
                empty_string,
                task.week.as_deref(),
                task.day.as_deref(),
                false,
                container_id,
            ],
        )?;

        Ok(Some(f.last_insert_rowid()))
    })
}

#[server]
pub async fn put_tasks(id: i64, info: String) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "UPDATE todo SET info = ?1 WHERE id = ?2",
            params![
                info,
                id,
            ],
        )?;
        Ok(())
    })
}

#[server]
pub async fn delete_tasks(id: i64) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute("DELETE FROM todo WHERE id = ?1", params![id])?;
        Ok(())
    })
}

#[server]
pub async fn get_tasks(
    container_title: String,
    current_week: String,
    current_day: Option<String>,
) -> Result<Vec<TaskResponse>, ServerFnError> {
    DB.with(|f| {
        let mut stmt;
        let mut rows;

        // Use JOIN to fetch todo based on the container title
        if let Some(day) = &current_day {
            stmt = f.prepare(
                "SELECT t.id, t.title, t.info, t.completed FROM todo t
                 JOIN containers c ON t.container_id = c.id
                 WHERE c.title = ?1 AND t.weeks = ?2 AND t.days = ?3",
            )?;
            rows = stmt.query(params![container_title, current_week, current_day])?;
        } else {
            stmt = f.prepare(
                "SELECT t.id, t.title, t.info, t.completed FROM todo t
                 JOIN containers c ON t.container_id = c.id
                 WHERE c.title = ?1 AND t.weeks = ?2 AND t.days IS NULL",
            )?;
            rows = stmt.query(params![container_title, current_week])?;
        }

        let mut tasks = Vec::new();
        while let Some(row) = rows.next()? {
            let task = TaskResponse {
                id: row.get(0)?,
                title: row.get(1)?,
                info: row.get(2)?,
                completed: row.get(3)?,
            };
            tasks.push(task);
        }
        Ok(tasks)
    })
}

#[server]
pub async fn get_task(id: i64) -> Result<Option<TaskResponse>, ServerFnError> {
    DB.with(|f| {
        let mut stmt = f.prepare(
            "SELECT id, title, info, completed
             FROM todo
             WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let task = TaskResponse {
                id: row.get(0)?,
                title: row.get(1)?,
                info: row.get(2)?,
                completed: row.get(3)?,
            };
            return Ok(Some(task));
        }
        Ok(None)
    })
}

#[server]
pub async fn put_completed(id: i64, completed: bool) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "UPDATE todo SET completed = ?1 WHERE id = ?2",
            params![
                completed,
                id,
            ],
        )?;
        Ok(())
    })
}
