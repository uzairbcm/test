#[cfg(test)]
mod backend_tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        response::Response,
    };
    use axum_backend::{
        models::{app_state::AppState, user_state::UserState},
        handlers::state_handlers::{get_user_state, update_user_state},
    };
    use sqlx::SqlitePool;
    use std::{path::Path, sync::Arc};
    use tempfile::tempdir;
    use tower::ServiceExt;

    // Helper function to create a test app state with a temporary directory and database
    async fn create_test_app_state() -> (Arc<AppState>, tempfile::TempDir) {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let temp_path = temp_dir.path();
        
        // Create data directories
        let data_dir = temp_path.join("data");
        std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        
        let csv_dir = data_dir.join("csv");
        std::fs::create_dir_all(&csv_dir).expect("Failed to create CSV directory");
        
        // Create in-memory database for testing
        let db = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        // Initialize schema
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_states (
                username TEXT PRIMARY KEY,
                text_entry TEXT NOT NULL,
                category1 TEXT NOT NULL,
                category2 TEXT NOT NULL,
                category3 TEXT NOT NULL,
                category4 TEXT NOT NULL,
                is_recording BOOLEAN NOT NULL,
                last_saved TEXT,
                last_data TEXT
            );
            
            CREATE TABLE IF NOT EXISTS data_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                text_entry TEXT NOT NULL,
                category1 TEXT NOT NULL,
                category2 TEXT NOT NULL,
                category3 TEXT NOT NULL,
                category4 TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                FOREIGN KEY(username) REFERENCES user_states(username)
            );
            "#,
        )
        .execute(&db)
        .await
        .unwrap();
        
        let app_state = Arc::new(AppState {
            db,
            csv_writers: Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
            data_dir,
        });
        
        (app_state, temp_dir)
    }
    
    // Helper function to create a test router
    fn app(state: Arc<AppState>) -> axum::Router {
        axum::Router::new()
            .route("/api/state/:username", axum::routing::get(get_user_state))
            .route("/api/state", axum::routing::post(update_user_state))
            .with_state(state)
    }
    
    #[sqlx::test]
    async fn test_update_and_get_user_state() {
        let (state, _temp_dir) = create_test_app_state().await;
        let app = app(state.clone());
        
        // Create a test user state
        let test_state = UserState {
            username: "testuser".to_string(),
            text_entry: "test text".to_string(),
            category1: "option1a".to_string(),
            category2: "option2a".to_string(),
            category3: "option3a".to_string(),
            category4: "option4a".to_string(),
            is_recording: false,
            last_saved: None,
            last_data: None,
        };
        
        // Update the user state
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/state")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&test_state).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        // Fetch the user state
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/state/testuser")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        // Check the response body
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let retrieved_state: UserState = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(retrieved_state.username, test_state.username);
        assert_eq!(retrieved_state.text_entry, test_state.text_entry);
        assert_eq!(retrieved_state.category1, test_state.category1);
        assert_eq!(retrieved_state.is_recording, test_state.is_recording);
    }
    
    #[sqlx::test]
    async fn test_recording_state() {
        let (state, temp_dir) = create_test_app_state().await;
        let app = app(state.clone());
        
        // Create a test user state with recording enabled
        let test_state = UserState {
            username: "recordinguser".to_string(),
            text_entry: "recording text".to_string(),
            category1: "option1b".to_string(),
            category2: "option2b".to_string(),
            category3: "option3b".to_string(),
            category4: "option4b".to_string(),
            is_recording: true,
            last_saved: Some("2023-01-01T00:00:00Z".to_string()),
            last_data: Some("test data".to_string()),
        };
        
        // Update the user state
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/state")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&test_state).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        // Check that a CSV file was created
        let csv_path = temp_dir.path().join("data").join("csv").join("recordinguser.csv");
        assert!(csv_path.exists());
        
        // Check that a database entry was created
        let log_entries = sqlx::query!(
            "SELECT COUNT(*) as count FROM data_logs WHERE username = ?",
            "recordinguser"
        )
        .fetch_one(&state.db)
        .await
        .unwrap();
        
        assert_eq!(log_entries.count, 1);
    }
}