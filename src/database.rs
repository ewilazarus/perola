use std::{fs, include_str, path::PathBuf};
use app_dirs::{get_app_dir, AppDataType, AppInfo};
use rusqlite::{Connection, Row, ToSql};
use fallible_streaming_iterator::FallibleStreamingIterator;

pub struct Database {
    connection_str: String,
}

impl Database {
    pub fn insert<T: DatabaseEntity<T>>(&self, entity: &T) {
        let connection = Connection::open(&self.connection_str).unwrap();
        let (sql, params) = entity.as_sql();
        connection.execute(sql, params).unwrap();
    }

    pub fn select<T: DatabaseEntity<T>>(&self, sql: &str, params: &[&dyn ToSql]) -> Vec<T> {
        let connection = Connection::open(&self.connection_str).unwrap();
        let mut statement = connection.prepare(sql).unwrap();
        let rows = statement.query(params).unwrap();

        let mut entities: Vec<T> = vec![];
        rows.for_each(|row| {
            entities.push(T::from_data(&row));
        }).unwrap();
        return entities;
    }
}

pub trait DatabaseEntity<T> {
    fn as_sql(&self) -> (&'static str, Vec<&dyn ToSql>);
    fn from_data(row: &Row<'_>) -> T;
}

fn db_path() -> PathBuf {
    let app_info = AppInfo {
        name: env!("CARGO_PKG_NAME"),
        author: env!("CARGO_PKG_AUTHORS"),
    };
    get_app_dir(AppDataType::UserData, &app_info, "perolas.db").unwrap()
}

fn spawn(path: &str) {
    let connection = Connection::open(path).unwrap();
    connection
        .execute_batch(include_str!("../resources/init.sql"))
        .unwrap();
}

pub fn teardown() {
    let path = db_path();
    let raw_path = path.to_str().unwrap();
    fs::remove_file(raw_path).unwrap();
}

pub fn setup() -> Database {
    let path = db_path();
    let raw_path = path.to_str().unwrap();
    if !path.exists() {
        spawn(&raw_path);
    }
    Database {
        connection_str: String::from(raw_path),
    }
}
