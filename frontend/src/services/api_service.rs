use reqwest::Client;
use crate::models::user_state::UserState;

#[derive(Clone)]
pub struct ApiService {
    client: Client,
    base_url: String,
}

impl ApiService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:3000/api".to_string(),
        }
    }

    pub async fn save_state(&self, state: &UserState) -> Result<(), reqwest::Error> {
        self.client
            .post(&format!("{}/state", self.base_url))
            .json(state)
            .send()
            .await?;
        Ok(())
    }

    pub async fn load_state(&self, username: &str) -> Result<Option<UserState>, reqwest::Error> {
        let response = self.client
            .get(&format!("{}/state/{}", self.base_url, username))
            .send()
            .await?;
        
        if response.status().is_success() {
            let state = response.json::<UserState>().await?;
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }
}