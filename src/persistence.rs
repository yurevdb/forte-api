#![allow(dead_code)]

use crate::types::{Channel, Message, User};
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};

/// Ensures that the database exists
pub async fn ensure_exists(pool: &SqlitePool, db_url: &str) -> Result<(), String> {
    if !std::path::Path::new("/tmp/forte")
        .try_exists()
        .unwrap_or(false)
    {
        match std::fs::create_dir_all("/tmp/forte") {
            Ok(_) => println!("Created /tmp/forte"),
            Err(error) => return Err(error.to_string()),
        }
    }

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => return Err(error.to_string()),
        }
    }

    match create_schema(pool).await {
        Ok(_) => (),
        Err(error) => return Err(error.to_string()),
    };

    Ok(())
}

/// Create the schema of the database
async fn create_schema(pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
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

    let result = sqlx::query(query).execute(pool).await;

    return result;
}

/// Get all the channels
pub async fn get_channels(pool: &SqlitePool) -> Vec<Channel> {
    let query = "SELECT * FROM channel";

    let channel_results: Vec<Channel> = sqlx::query_as::<_, Channel>(query)
        .fetch_all(pool)
        .await
        .unwrap();

    return channel_results;
}

/// Save the given channel
pub async fn insert_channel(
    pool: &SqlitePool,
    channel: &Channel,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let query = "INSERT INTO channel (name) VALUES(?)";

    let result = sqlx::query(query)
        .bind(channel.name.as_str())
        .execute(pool)
        .await;

    return result;
}

/// Delete the channel with the given id
pub async fn delete_channel(pool: &SqlitePool, id: u32) -> Result<SqliteQueryResult, sqlx::Error> {
    let query = "DELETE FROM channel WHERE id = ?";

    let result = sqlx::query(query).bind(id).execute(pool).await;

    return result;
}

/// Creates a user in the database
pub async fn create_user(pool: &SqlitePool, user: &User) -> Result<SqliteQueryResult, sqlx::Error> {
    let query = "INSERT INTO user (name) VALUES(?)";

    let result = sqlx::query(query)
        .bind(user.name.as_str())
        .execute(pool)
        .await;

    return result;
}

/// Creates a message in the database
pub async fn create_message(
    pool: &SqlitePool,
    message: &Message,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let query = "INSERT INTO message (content, channel_id, user_id) VALUES(?, ?, ?)";

    let result = sqlx::query(query)
        .bind(message.content.as_str())
        .bind(message.channel_id)
        .bind(message.user_id)
        .execute(pool)
        .await;

    return result;
}

/// Get messages from a channel
pub async fn get_channel_messages(pool: &SqlitePool, id: u32) -> Vec<Message> {
    let query = "SELECT * FROM message WHERE channel_id = ?";

    let messages: Vec<Message> = sqlx::query_as::<_, Message>(query)
        .bind(id)
        .fetch_all(pool)
        .await
        .unwrap();

    return messages;
}

/// Get info for the user with the given id
pub async fn get_user_info(pool: &SqlitePool, id: u32) -> User {
    let query = "SELECT * FROM user WHERE id = ?";

    let user_info: User = sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap();

    return user_info;
}
