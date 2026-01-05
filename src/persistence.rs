use crate::messages::{LoadError, SavedState};
use std::path::PathBuf;

pub fn data_file_path() -> PathBuf {
    let data_dir = directories::ProjectDirs::from("com", "inventory", "app")
        .expect("Failed to get data directory")
        .data_dir()
        .to_path_buf();

    std::fs::create_dir_all(&data_dir).ok();
    data_dir.join("inventory.json")
}

pub async fn load_state() -> Result<SavedState, LoadError> {
    let path = data_file_path();

    if !path.exists() {
        return Err(LoadError::FileNotFound);
    }

    let contents = tokio::fs::read_to_string(path)
        .await
        .map_err(|_| LoadError::FileNotFound)?;

    serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
}

pub async fn save_state(state: &SavedState) -> Result<(), std::io::Error> {
    let path = data_file_path();
    let json = serde_json::to_string_pretty(state)?;
    tokio::fs::write(path, json).await
}
