use iced::Task;
use crate::{InventoryApp, Message};

impl InventoryApp {
    pub fn handle_export_audit_log(&self) -> Task<Message> {
        let csv_content = self.audit_log.export_to_csv();
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("audit_log_{}.csv", timestamp);
        
        Task::perform(
            async move {
                let export_path = directories::UserDirs::new()
                    .and_then(|dirs| dirs.desktop_dir().map(|p| p.to_path_buf()))
                    .or_else(|| {
                        directories::UserDirs::new()
                            .map(|dirs| dirs.home_dir().to_path_buf())
                    })
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                
                let file_path = export_path.join(filename);
                std::fs::write(file_path, csv_content).ok();
            },
            |_| Message::Save,
        )
    }
}

