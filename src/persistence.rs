#![allow(dead_code)]

use sqlx::{migrate::MigrateDatabase, Sqlite};

const DB_URL: &str = "sqlite://db/data.db";

/// Ensures that the database exists
pub async fn ensure_exists() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}
