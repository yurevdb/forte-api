use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// A message channel
#[derive(Serialize, Deserialize, FromRow)]
pub struct Channel {
    pub id: Option<i32>,
    pub name: String,
}

/// A message
#[derive(Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Option<i32>,
    pub user_id: i32,
    pub channel_id: i32,
    pub content: String,
}

/// A user
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
}
