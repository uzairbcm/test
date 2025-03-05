use std::{fs::{self, File}, io, path::Path};
use csv::Writer;

pub struct CsvExporter {
    base_dir: String,
}

impl CsvExporter {
    pub fn new(base_dir: &str) -> Self {
        Self {
            base_dir: base_dir.to_string(),
        }
    }
    
    pub fn ensure_dir_exists(&self) -> io::Result<()> {
        fs::create_dir_all(&self.base_dir)?;
        Ok(())
    }
    
    pub fn get_writer(&self, filename: &str) -> io::Result<Writer<File>> {
        self.ensure_dir_exists()?;
        
        let path = Path::new(&self.base_dir).join(filename);
        let file_exists = path.exists();
        
        let file = File::options()
            .create(true)
            .append(true)
            .open(&path)?;
            
        let mut writer = csv::WriterBuilder::new().from_writer(file);
        
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
        
        Ok(writer)
    }
}