use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserState {
    pub username: String,
    pub text_entry: String,
    pub category1: String,
    pub category2: String,
    pub category3: String,
    pub category4: String,
    pub is_recording: bool,
    pub last_saved: Option<String>,
    pub last_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DataLog {
    pub id: Option<i64>,
    pub username: String,
    pub text_entry: String,
    pub category1: String,
    pub category2: String,
    pub category3: String,
    pub category4: String,
    pub timestamp: String,
}