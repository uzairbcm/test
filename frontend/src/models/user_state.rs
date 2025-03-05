use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl Default for UserState {
    fn default() -> Self {
        Self {
            username: String::new(),
            text_entry: String::new(),
            category1: String::new(),
            category2: String::new(),
            category3: String::new(),
            category4: String::new(),
            is_recording: false,
            last_saved: None,
            last_data: None,
        }
    }
}