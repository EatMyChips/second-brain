use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::super::init_database::DB;
#[cfg(feature = "server")]
use rusqlite::{params, Connection, Result as SqlResult, ToSql};
use crate::NewTask;

/* Check object endpoints */
#[server]
pub async fn post_new_check() -> Result<Option<i64>, ServerFnError> {
    let empty_string: String = String::new();
    DB.with(|f| {
        f.execute(
            "INSERT INTO rewards (title) VALUES (?1)",
            params![
                empty_string,
            ],
        )?;
        Ok(Some(f.last_insert_rowid()))
    })
}

#[server]
pub async fn put_check_info() -> Result<Option<i64>, ServerFnError> {
    Ok(Some(1))
}

#[server]
pub async fn get_check_info() -> Result<Option<i64>, ServerFnError> {
    Ok(Some(1))
}

/* TODO:: Calls delete for every weekly entry for this check object */
#[server]
pub async fn delete_check() -> Result<Option<i64>, ServerFnError> {
    Ok(Some(1))
}


/* Check weekly data endpoints */
#[server]
pub async fn put_check_week_data() -> Result<Option<i64>, ServerFnError> {
    Ok(Some(1))
}

/* TODO:: Must check if data exists for this week, if it doesnt create a new entry */
#[server]
pub async fn get_check_week_data() -> Result<Option<i64>, ServerFnError> {
    Ok(Some(1))
}

/* TODO:: Gets called when check object is deleted */
#[server]
pub async fn delete_check_week() -> Result<Option<i64>, ServerFnError> {
    Ok(Some(1))
}