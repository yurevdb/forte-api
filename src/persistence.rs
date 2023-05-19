#![allow(dead_code)]

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, sqlite::SqliteQueryResult};
use crate::types::Channel;

/// Database url
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

    match create_schema(DB_URL).await {
        Ok(_) => println!("Schema created"),
        Err(error) => panic!("error: {}", error),
    };
}

/// Create the schema of the database
async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;

    let query = "CREATE TABLE IF NOT EXISTS channel 
                 (id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, 
                  name VARCHAR(256) NOT NULL);";

    let result = sqlx::query(query).execute(&pool).await;

    pool.close().await;

    return result;
}

/// Get all the channels
pub async fn get_channels() -> Vec<Channel> {
    let pool = SqlitePool::connect(DB_URL).await.expect("Connection could not be established.");

    let query = "SELECT rowid, name FROM channel";

    let channel_results: Vec<Channel> = sqlx::query_as::<_, Channel>(query).fetch_all(&pool).await.unwrap();

    return channel_results;
}

/// Save the given channel
pub async fn insert_channel(channel: &Channel) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(DB_URL).await?;

    let query = format!("INSERT INTO channel (name) VALUES('{}')", channel.name);

    let result = sqlx::query(query.as_str()).execute(&pool).await;

    pool.close().await;

    return result;
}
