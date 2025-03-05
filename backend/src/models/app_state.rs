use std::{collections::HashMap, fs::{self, File}, path::PathBuf, sync::{Arc, RwLock}};
use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::sync::Mutex;
use csv::Writer;

pub struct AppState {
    pub db: Pool<Sqlite>,
    pub csv_writers: Arc<RwLock<HashMap<String, Mutex<Writer<File>>>>>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Create data directory if it doesn't exist
        let data_dir = PathBuf::from("data");
        fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        
        // Create CSV directory if it doesn't exist
        let csv_dir = data_dir.join("csv");
        fs::create_dir_all(&csv_dir).expect("Failed to create CSV directory");
        
        // Initialize the database connection
        let db = SqlitePool::connect("sqlite:data/app.db").await?;
        
        // Initialize the database schema
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
        .await?;
        
        Ok(Self {
            db,
            csv_writers: Arc::new(RwLock::new(HashMap::new())),
            data_dir,
        })
    }
    
    pub async fn get_csv_writer(&self, username: &str) -> std::io::Result<Mutex<Writer<File>>> {
        let writers = self.csv_writers.read().unwrap();
        
        if let Some(writer) = writers.get(username) {
            return Ok(writer.clone());
        }
        
        drop(writers); // Release the read lock
        
        let mut writers = self.csv_writers.write().unwrap();
        let csv_path = self.data_dir.join("csv").join(format!("{}.csv", username));
        
        let file_exists = csv_path.exists();
        let file = File::options()
            .create(true)
            .append(true)
            .open(&csv_path)?;
        
        let mut writer = csv::WriterBuilder::new().from_writer(file);
        
        // Write headers if the file is new
        if !file_exists {
            writer.write_record(&[
                "username",
                "text_entry",
                "category1",
                "category2",
                "category3",
                "category4",
                "timestamp",
            ])?;
            writer.flush()?;
        }
        
        let mutex = Mutex::new(writer);
        writers.insert(username.to_string(), mutex.clone());
        
        Ok(mutex)
    }
}