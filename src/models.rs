use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::MySqlPool;

pub struct AppState {
    pub db_pool: MySqlPool,
}

#[derive(serde::Serialize, serde::Deserialize, FromRow)]
pub struct ShortUrl {
    pub id: i32,
    pub original_url: String,
    pub short_code: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub access_count: i32,
}

#[derive(serde::Deserialize)]
pub struct ShortUrlRequest {
    pub url: String,
}
