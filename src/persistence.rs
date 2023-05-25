use crate::types::{Channel, Message, User};
use eyre::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions, PgQueryResult};
use sqlx::Postgres;

/// Database url
const DB_PG: &str = "postgresql://postgres:postgres@db.api/forte";

/// Ensures that the database exists
pub async fn ensure_exists() -> Result<PgPool> {
    if !Postgres::database_exists(DB_PG).await.unwrap_or(false) {
        println!("Creating database {}", DB_PG);
        Postgres::create_database(DB_PG).await?
    }

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(DB_PG)
        .await?;

    create_schema(&pool).await?;

    Ok(pool)
}

/// Create the schema of the database
async fn create_schema(pool: &PgPool) -> Result<()> {
    let channels =
        "CREATE TABLE IF NOT EXISTS channels (id SERIAL PRIMARY KEY, name VARCHAR(256) NOT NULL);";
    let users =
        "CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name VARCHAR(128) NOT NULL);";
    let messages = "CREATE TABLE IF NOT EXISTS messages (
                        id          SERIAL PRIMARY KEY,
                        content     TEXT,
                        user_id     INTEGER NOT NULL,
                        channel_id  INTEGER NOT NULL,
                        FOREIGN KEY(user_id) REFERENCES users(id),
                        FOREIGN KEY(channel_id) REFERENCES channels(id)
                    );";

    sqlx::query(channels).execute(pool).await?;
    sqlx::query(users).execute(pool).await?;
    sqlx::query(messages).execute(pool).await?;

    Ok(())
}

/// Get all the channels
pub async fn get_channels(pool: &PgPool) -> Result<Vec<Channel>> {
    let query = "SELECT * FROM channels";

    let vec = sqlx::query_as::<_, Channel>(query).fetch_all(pool).await?;

    Ok(vec)
}

/// Save the given channel
pub async fn insert_channel(pool: &PgPool, channel: &Channel) -> Result<PgQueryResult> {
    let query = "INSERT INTO channels (name) VALUES($1)";

    let result = sqlx::query(query)
        .bind(channel.name.as_str())
        .execute(pool)
        .await?;

    Ok(result)
}

/// Delete the channel with the given id
pub async fn delete_channel(pool: &PgPool, id: i32) -> Result<PgQueryResult> {
    let query = "DELETE FROM channels WHERE id = $1";

    let result = sqlx::query(query).bind(id).execute(pool).await?;

    Ok(result)
}

/// Creates a user in the database
pub async fn create_user(pool: &PgPool, user: &User) -> Result<PgQueryResult> {
    let query = "INSERT INTO users (name) VALUES($1)";

    let result = sqlx::query(query)
        .bind(user.name.as_str())
        .execute(pool)
        .await?;

    Ok(result)
}

/// Creates a message in the database
pub async fn create_message(pool: &PgPool, message: &Message) -> Result<PgQueryResult> {
    let query = "INSERT INTO messages (content, channel_id, user_id) VALUES($1, $2, $3)";

    let result = sqlx::query(query)
        .bind(message.content.as_str())
        .bind(message.channel_id)
        .bind(message.user_id)
        .execute(pool)
        .await?;

    Ok(result)
}

/// Get messages from a channel
pub async fn get_channel_messages(pool: &PgPool, id: i32) -> Result<Vec<Message>> {
    let query = "SELECT * FROM messages WHERE channel_id = $1";

    let messages: Vec<Message> = sqlx::query_as::<_, Message>(query)
        .bind(id)
        .fetch_all(pool)
        .await?;

    Ok(messages)
}

/// Get info for the user with the given id
pub async fn get_user_info(pool: &PgPool, id: i32) -> Result<User> {
    let query = "SELECT * FROM users WHERE id = $1";

    let user_info: User = sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(user_info)
}
