#![allow(dead_code)]

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, sqlite::SqliteQueryResult};
use crate::types::{Channel, User, Message};

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

    let query = "
        CREATE TABLE IF NOT EXISTS channel 
        (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    VARCHAR(256) NOT NULL
        );
        CREATE TABLE IF NOT EXISTS user 
        (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    VARCHAR(128) NOT NULL
        );
        CREATE TABLE IF NOT EXISTS message 
        (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            content     TEXT,
            user_id     INTEGER NOT NULL,
            channel_id  INTEGER NOT NULL,
            FOREIGN KEY(user_id) REFERENCES user(id)
            FOREIGN KEY(channel_id) REFERENCES channel(id)
        );
    ";

    let result = sqlx::query(query).execute(&pool).await;

    pool.close().await;

    return result;
}

/// Get all the channels
pub async fn get_channels() -> Vec<Channel> {
    let pool = SqlitePool::connect(DB_URL).await.expect("Connection could not be established.");

    let query = "SELECT * FROM channel";

    let channel_results: Vec<Channel> = sqlx::query_as::<_, Channel>(query).fetch_all(&pool).await.unwrap();

    return channel_results;
}

/// Save the given channel
pub async fn insert_channel(channel: &Channel) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(DB_URL).await?;

    let query = "INSERT INTO channel (name) VALUES(?)";

    let result = sqlx::query(query).bind(channel.name.as_str()).execute(&pool).await;

    pool.close().await;

    return result;
}

/// Delete the channel with the given id
pub async fn delete_channel(id: u32) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(DB_URL).await?;

    let query = "DELETE FROM channel WHERE id = ?";

    let result = sqlx::query(query).bind(id).execute(&pool).await;

    pool.close().await;

    return result;
}

/// Creates a user in the database
pub async fn create_user(user: &User) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(DB_URL).await?;

    let query = "INSERT INTO user (name) VALUES(?)";

    let result = sqlx::query(query).bind(user.name.as_str()).execute(&pool).await;

    pool.close().await;

    return result;
}

/// Creates a message in the database
pub async fn create_message(message: &Message) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(DB_URL).await?;

    let query = "INSERT INTO message (content, channel_id, user_id) VALUES(?, ?, ?)";

    let result = sqlx::query(query)
        .bind(message.content.as_str())
        .bind(message.channel.id)
        .bind(message.user.id)
        .execute(&pool)
        .await;

    pool.close().await;

    return result;
}
