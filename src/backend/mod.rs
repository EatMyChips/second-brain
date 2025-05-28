use dioxus::prelude::*;

#[cfg(feature = "server")]
use rusqlite::ToSql;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("todo.db").expect("Failed to open database");
        println!("Database connection created!");
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;

            CREATE TABLE containers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL
            );

            CREATE TABLE tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                info TEXT NOT NULL,
                weeks DATE,
                days DATE,
                container_id INTEGER NOT NULL,
                FOREIGN KEY (container_id) REFERENCES containers(id)
            );

            -- Insert sample data
            INSERT INTO containers (title) VALUES
            ('Work'),
            ('Personal'),
            ('Fitness'),
            ('Learning');",
        ).unwrap();

        // Return the connection
        conn
    };
}


#[server]
pub async fn save_task(
    title: String,
    info: String,
    weeks: Option<String>, // Dates are optional
    days: Option<String>,
    container_id: i32,
) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "INSERT INTO tasks (title, info, weeks, days, container_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[
                &title as &dyn ToSql,
                &info as &dyn ToSql,
                &weeks.as_ref().map(|s| s.as_str()) as &dyn ToSql,
                &days.as_ref().map(|s| s.as_str()) as &dyn ToSql,
                &container_id as &dyn ToSql,
            ],
        )
    })?;
    Ok(())
}

