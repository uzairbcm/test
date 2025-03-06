use csv::Writer;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tokio::sync::Mutex;

pub struct AppState {
    pub db: Pool<Sqlite>,
    pub csv_writers: Arc<RwLock<HashMap<String, Arc<Mutex<Writer<File>>>>>>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Print current working directory for debugging
        let current_dir = std::env::current_dir().unwrap_or_default();
        println!("Current working directory: {}", current_dir.display());
        
        // Create data directory with absolute path if it doesn't exist
        let data_dir = current_dir.join("data");
        println!("Data directory path: {}", data_dir.display());
        
        // Create directories with error reporting
        match fs::create_dir_all(&data_dir) {
            Ok(_) => println!("Data directory created or already exists"),
            Err(e) => eprintln!("Failed to create data directory: {}", e),
        };
        
        // Create CSV directory if it doesn't exist
        let csv_dir = data_dir.join("csv");
        match fs::create_dir_all(&csv_dir) {
            Ok(_) => println!("CSV directory created or already exists"),
            Err(e) => eprintln!("Failed to create CSV directory: {}", e),
        };
        
        // Generate the database file path with absolute path
        let db_path = data_dir.join("app.db");
        println!("Database path: {}", db_path.display());
        
        // Check if we can create a test file in the directory
        let test_file_path = data_dir.join("test_write.txt");
        match std::fs::File::create(&test_file_path) {
            Ok(_) => {
                println!("Write test successful");
                // Clean up
                let _ = std::fs::remove_file(test_file_path);
            },
            Err(e) => eprintln!("Write test failed: {}", e),
        }
        
        // Construct proper SQLite URL with absolute path
        let db_path = data_dir.join("app.db");
        let db_url = format!("sqlite://{}", db_path.to_string_lossy());
        println!("Database URL: {}", db_url);

        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            println!("Creating database {}", &db_url);
            match Sqlite::create_database(&db_url).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        }
        // Initialize the database connection
        println!("Attempting to connect to the database...");
        let db = SqlitePool::connect(&db_url).await?;
        println!("Database connection established successfully");

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
    // In backend/src/models/app_state.rs, change the get_csv_writer method

    pub async fn get_csv_writer(
        &self,
        username: &str,
    ) -> std::io::Result<Arc<Mutex<Writer<File>>>> {
        let writers = self.csv_writers.read().unwrap();

        if let Some(writer) = writers.get(username) {
            return Ok(writer.clone());
        }

        drop(writers); // Release the read lock

        let mut writers = self.csv_writers.write().unwrap();
        let csv_path = self.data_dir.join("csv").join(format!("{}.csv", username));

        let file_exists = csv_path.exists();
        let file = File::options().create(true).append(true).open(&csv_path)?;

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

        // Create an Arc<Mutex<...>> instead of just Mutex<...>
        let arc_mutex = Arc::new(Mutex::new(writer));
        writers.insert(username.to_string(), arc_mutex.clone());

        Ok(arc_mutex)
    }
}
