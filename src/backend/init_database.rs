use rusqlite::{params, Connection, ToSql, Result as SqlResult};


thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("todo.db").expect("Failed to open database");
        println!("Database connection created!");
        conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS containers (
                id INTEGER PRIMARY KEY NOT NULL,
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
            -- INSERT OR IGNORE INTO containers (id, title) VALUES (1, 'todays-tasks'), (2, 'professional'), (3, 'personal');
            ",
        ).unwrap();

        // Return the connection
        conn
    };
}