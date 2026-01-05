use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditAction {
    // Item actions
    ItemCreated,
    ItemUpdated,
    ItemDeleted,

    // Note actions
    NoteCreated,
    NoteUpdated,
    NoteDeleted,

    // User actions
    UserLogin,
    UserLogout,
    UserCreated,
    UserUpdated,
    UserDeleted,

    // Settings actions
    SettingsChanged,

    // Data actions
    DataExported,
    DataImported,
    DataCleared,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::ItemCreated => write!(f, "Item Created"),
            AuditAction::ItemUpdated => write!(f, "Item Updated"),
            AuditAction::ItemDeleted => write!(f, "Item Deleted"),
            AuditAction::NoteCreated => write!(f, "Note Created"),
            AuditAction::NoteUpdated => write!(f, "Note Updated"),
            AuditAction::NoteDeleted => write!(f, "Note Deleted"),
            AuditAction::UserLogin => write!(f, "User Login"),
            AuditAction::UserLogout => write!(f, "User Logout"),
            AuditAction::UserCreated => write!(f, "User Created"),
            AuditAction::UserUpdated => write!(f, "User Updated"),
            AuditAction::UserDeleted => write!(f, "User Deleted"),
            AuditAction::SettingsChanged => write!(f, "Settings Changed"),
            AuditAction::DataExported => write!(f, "Data Exported"),
            AuditAction::DataImported => write!(f, "Data Imported"),
            AuditAction::DataCleared => write!(f, "Data Cleared"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: i64,
    pub user_id: String,
    pub username: String,
    pub action: AuditAction,
    pub entity_type: String, // "item", "note", "user", "settings"
    pub entity_id: Option<String>,
    pub details: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

impl AuditEntry {
    pub fn new(
        user_id: String,
        username: String,
        action: AuditAction,
        entity_type: String,
        entity_id: Option<String>,
        details: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now().timestamp(),
            user_id,
            username,
            action,
            entity_type,
            entity_id,
            details,
            old_value: None,
            new_value: None,
        }
    }

    pub fn with_values(mut self, old_value: Option<String>, new_value: Option<String>) -> Self {
        self.old_value = old_value;
        self.new_value = new_value;
        self
    }

    pub fn formatted_timestamp(&self) -> String {
        chrono::DateTime::from_timestamp(self.timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditLog {
    entries: Vec<AuditEntry>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: AuditEntry) {
        self.entries.push(entry);

        // Keep only last 1000 entries to prevent unbounded growth
        if self.entries.len() > 1000 {
            self.entries.drain(0..self.entries.len() - 1000);
        }
    }

    pub fn get_entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    pub fn filter_by_user(&self, user_id: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.user_id == user_id)
            .collect()
    }

    pub fn filter_by_action(&self, action: &AuditAction) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| &e.action == action)
            .collect()
    }

    pub fn filter_by_entity(&self, entity_id: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.entity_id.as_deref() == Some(entity_id))
            .collect()
    }

    pub fn filter_by_date_range(&self, start: i64, end: i64) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    pub fn get_recent(&self, count: usize) -> Vec<&AuditEntry> {
        let start_index = if self.entries.len() > count {
            self.entries.len() - count
        } else {
            0
        };
        self.entries[start_index..].iter().rev().collect()
    }

    pub fn export_to_csv(&self) -> String {
        let mut csv = String::from("ID,Timestamp,User,Action,Entity Type,Entity ID,Details\n");

        for entry in &self.entries {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                entry.id,
                entry.formatted_timestamp(),
                entry.username,
                entry.action,
                entry.entity_type,
                entry.entity_id.as_deref().unwrap_or("N/A"),
                entry.details.replace(',', ";") // Escape commas
            ));
        }

        csv
    }
}
