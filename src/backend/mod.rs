use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use rusqlite::{params, Connection, ToSql, Result as SqlResult};

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

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("todo.db").expect("Failed to open database");
        println!("Database connection created!");
        conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS containers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                info TEXT NOT NULL,
                weeks DATE,
                days DATE,
                container_id INTEGER NOT NULL,
                FOREIGN KEY (container_id) REFERENCES containers(id)
            );

            -- Insert sample data
            INSERT OR IGNORE  INTO containers (title) VALUES
            ('Work'),
            ('Personal'),
            ('Fitness'),
            ('Learning');
            ",
        ).unwrap();

        // Return the connection
        conn
    };
}

#[server]
pub async fn post_tasks(task: TaskInput) -> Result<Option<Task>, ServerFnError> {
    DB.with(|f| {
        f.execute(
            "INSERT INTO tasks (title, info, weeks, days, container_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task.title,
                task.info,
                task.week.as_deref(),
                task.day.as_deref(),
                task.container_id,
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
pub async fn get_tasks(id: i32) -> Result<Vec<Task>, ServerFnError> {
    DB.with(|f| {
        let mut stmt = f.prepare(
            "SELECT id, title, info, weeks, days, container_id FROM tasks WHERE container_id = ?1"
        )?;
        let mut rows = stmt.query(params![id])?;

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


