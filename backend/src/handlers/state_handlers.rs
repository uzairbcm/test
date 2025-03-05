use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::models::{app_state::AppState, user_state::UserState};

pub async fn get_user_state(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> Result<Json<UserState>, StatusCode> {
    let result = sqlx::query_as::<_, UserState>(
        r#"
        SELECT * FROM user_states
        WHERE username = ?
        "#,
    )
    .bind(&username)
    .fetch_optional(&state.db)
    .await;
    
    match result {
        Ok(Some(user_state)) => Ok(Json(user_state)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_user_state(
    State(state): State<Arc<AppState>>,
    Json(user_state): Json<UserState>,
) -> StatusCode {
    // Update the user state in the database
    let result = sqlx::query(
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
    .bind(&user_state.username)
    .bind(&user_state.text_entry)
    .bind(&user_state.category1)
    .bind(&user_state.category2)
    .bind(&user_state.category3)
    .bind(&user_state.category4)
    .bind(&user_state.is_recording)
    .bind(&user_state.last_saved)
    .bind(&user_state.last_data)
    .execute(&state.db)
    .await;
    
    // If we're recording, log the data
    if user_state.is_recording {
        // Get the current timestamp
        let now: DateTime<Utc> = Utc::now();
        let timestamp = now.to_rfc3339();
        
        // Log to the database
        let log_result = sqlx::query(
            r#"
            INSERT INTO data_logs (
                username, text_entry, category1, category2, category3, category4, timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&user_state.username)
        .bind(&user_state.text_entry)
        .bind(&user_state.category1)
        .bind(&user_state.category2)
        .bind(&user_state.category3)
        .bind(&user_state.category4)
        .bind(&timestamp)
        .execute(&state.db)
        .await;
        
        if log_result.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        
        // Log to CSV file
        match state.get_csv_writer(&user_state.username).await {
            Ok(writer_mutex) => {
                let mut writer = writer_mutex.lock().await;
                let record_result = writer.write_record(&[
                    &user_state.username,
                    &user_state.text_entry,
                    &user_state.category1,
                    &user_state.category2,
                    &user_state.category3,
                    &user_state.category4,
                    &timestamp,
                ]);
                
                if let Err(_) = record_result.and_then(|_| writer.flush()) {
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    
    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}