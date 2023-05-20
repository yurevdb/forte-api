use serde::{Serialize, Deserialize};
use sqlx::{FromRow, sqlite::SqliteRow, Row};

/// A message channel
#[derive(Serialize, Deserialize)]
pub struct Channel {
    pub id: Option<u32>,
    pub name: String,
    pub messages: Option<Vec<Message>>,
    pub users: Option<Vec<User>>,
}

impl FromRow<'_, SqliteRow> for Channel {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            messages: None,
            users: None,
        })
    }
}

/// A message
#[derive(Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Option<u64>,
    pub user: User,
    pub channel: Channel,
    pub content: String,
}

/// An user
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<u32>,
    pub name: String
}
