use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Note {
    pub fn new(title: String) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content: String::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now().timestamp();
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.update_timestamp();
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.update_timestamp();
    }
}
