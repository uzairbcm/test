use sqlx::{Pool, Sqlite};
use crate::models::user_state::{UserState, DataLog};

pub struct SqliteRepository {
    pool: Pool<Sqlite>,
}

impl SqliteRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    pub async fn get_user_state(&self, username: &str) -> Result<Option<UserState>, sqlx::Error> {
        sqlx::query_as::<_, UserState>(
            "SELECT * FROM user_states WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
    }
    
    pub async fn save_user_state(&self, state: &UserState) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO user_states (
                username, text_entry, category1, category2, category3, category4,
                is_recording, last_saved, last_data
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(username) DO UPDATE SET
                text_entry = excluded.text_entry,
                category1 = excluded.category1,
                category2 = excluded.category2,
                category3 = excluded.category3,
                category4 = excluded.category4,
                is_recording = excluded.is_recording,
                last_saved = excluded.last_saved,
                last_data = excluded.last_data
            "#,
        )
        .bind(&state.username)
        .bind(&state.text_entry)
        .bind(&state.category1)
        .bind(&state.category2)
        .bind(&state.category3)
        .bind(&state.category4)
        .bind(&state.is_recording)
        .bind(&state.last_saved)
        .bind(&state.last_data)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn log_data_entry(&self, log: &DataLog) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO data_logs (
                username, text_entry, category1, category2, category3, category4, timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&log.username)
        .bind(&log.text_entry)
        .bind(&log.category1)
        .bind(&log.category2)
        .bind(&log.category3)
        .bind(&log.category4)
        .bind(&log.timestamp)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_user_logs(&self, username: &str) -> Result<Vec<DataLog>, sqlx::Error> {
        sqlx::query_as::<_, DataLog>(
            "SELECT * FROM data_logs WHERE username = ? ORDER BY timestamp DESC"
        )
        .bind(username)
        .fetch_all(&self.pool)
        .await
    }
}