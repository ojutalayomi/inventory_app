use iced::Task;
use crate::{InventoryApp, Message};
use crate::messages::{AppTheme, LayoutStyle, SavedState};
use crate::audit::{AuditAction, AuditEntry};

impl InventoryApp {
    pub fn handle_toggle_auto_save(&mut self) -> Task<Message> {
        self.settings.auto_save_enabled = !self.settings.auto_save_enabled;
        self.auto_save()
    }

    pub fn handle_auto_save_interval_changed(&mut self, value: String) -> Task<Message> {
        self.settings_interval_input = value.clone();
        if let Ok(interval) = value.parse::<u32>() {
            if interval > 0 && interval <= 300 {
                self.settings.auto_save_interval = interval;
                return self.auto_save();
            }
        }
        Task::none()
    }

    pub fn handle_default_category_changed(&mut self, value: String) -> Task<Message> {
        self.settings_category_input = value.clone();
        self.settings.default_category = value;
        self.auto_save()
    }

    pub fn handle_theme_changed(&mut self, theme: AppTheme) -> Task<Message> {
        self.settings.theme = theme;
        self.auto_save()
    }

    pub fn handle_toggle_loading_screen(&mut self) -> Task<Message> {
        self.settings.show_loading_screen = !self.settings.show_loading_screen;
        self.auto_save()
    }

    pub fn handle_layout_style_changed(&mut self, style: LayoutStyle) -> Task<Message> {
        self.settings.layout_style = style;
        self.auto_save()
    }

    pub fn handle_export_data(&mut self) -> Task<Message> {
        // Log export
        if let Some(session) = &self.session {
            let audit_entry = AuditEntry::new(
                session.user_id.clone(),
                session.username.clone(),
                AuditAction::DataExported,
                "data".to_string(),
                None,
                "Exported all data to JSON file".to_string(),
            );
            self.audit_log.add_entry(audit_entry);
        }
        
        let state = SavedState {
            items: self.items.clone(),
            notes: self.notes.clone(),
            calculator_position: self.calculator.position,
            settings: self.settings.clone(),
            auth_store: self.auth_store.clone(),
            audit_log: self.audit_log.clone(),
            alert_manager: self.alert_manager.clone(),
        };
        let task = Task::perform(
            async move {
                let json = serde_json::to_string_pretty(&state).unwrap_or_default();
                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                let filename = format!("inventory_export_{}.json", timestamp);
                
                // Get desktop path or fallback to home
                let export_path = directories::UserDirs::new()
                    .and_then(|dirs| dirs.desktop_dir().map(|p| p.to_path_buf()))
                    .or_else(|| {
                        directories::UserDirs::new()
                            .map(|dirs| dirs.home_dir().to_path_buf())
                    })
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                
                let file_path = export_path.join(filename);
                std::fs::write(file_path, json).ok();
            },
            |_| Message::Save,
        );
        Task::batch(vec![self.auto_save(), task])
    }

    pub fn handle_open_import_file_picker(&mut self) -> Task<Message> {
        self.import_file_picker_open = true;
        Task::perform(
            async {
                let home_dir = directories::UserDirs::new()
                    .map(|dirs| dirs.home_dir().to_path_buf())
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                
                let file = rfd::FileDialog::new()
                    .add_filter("JSON files", &["json"])
                    .set_directory(&home_dir)
                    .pick_file();
                
                Message::ImportFileSelected(file)
            },
            |msg| msg,
        )
    }

    pub fn handle_import_data(&self) -> Task<Message> {
        use crate::persistence;
        
        // Legacy handler - kept for backward compatibility
        // For now, users can manually copy their exported file to the data location
        Task::perform(
            async move {
                // Try to read from common locations
                let home = directories::UserDirs::new()
                    .map(|dirs| dirs.home_dir().to_path_buf())
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                
                let import_path = home.join("Downloads").join("inventory_import.json");
                
                if let Ok(contents) = std::fs::read_to_string(&import_path) {
                    if let Ok(state) = serde_json::from_str::<SavedState>(&contents) {
                        persistence::save_state(&state).await.ok();
                    }
                }
            },
            |_| Message::Save,
        )
    }

    pub fn handle_import_file_selected(&mut self, file_path: Option<std::path::PathBuf>) -> Task<Message> {
        
        // Reset file picker state
        self.import_file_picker_open = false;
        
        // If user canceled, just return
        let path = match file_path {
            Some(p) => p,
            None => {
                self.import_error = None; // Clear any previous errors
                return Task::none();
            }
        };
        
        // Read and parse the file
        let file_contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(e) => {
                self.import_error = Some(format!("Failed to read file: {}", e));
                return Task::none();
            }
        };
        
        let imported_state: SavedState = match serde_json::from_str(&file_contents) {
            Ok(state) => state,
            Err(e) => {
                self.import_error = Some(format!("Invalid JSON format: {}", e));
                return Task::none();
            }
        };
        
        // Log import action
        if let Some(session) = &self.session {
            let audit_entry = AuditEntry::new(
                session.user_id.clone(),
                session.username.clone(),
                AuditAction::DataImported,
                "data".to_string(),
                None,
                format!("Imported data from {}", path.display()),
            );
            self.audit_log.add_entry(audit_entry);
        }
        
        // Replace current data with imported data (Option A from plan)
        // This completely replaces existing data
        self.items = imported_state.items.clone();
        self.filtered_items = self.search_filter.apply(&self.items);
        self.notes = imported_state.notes.clone();
        self.settings = imported_state.settings.clone();
        self.auth_store = imported_state.auth_store.clone();
        self.audit_log = imported_state.audit_log.clone();
        self.alert_manager = imported_state.alert_manager.clone();
        self.alert_manager.update_from_inventory(&self.items);
        
        // Update settings inputs
        self.settings_interval_input = self.settings.auto_save_interval.to_string();
        self.settings_category_input = self.settings.default_category.clone();
        
        // Update calculator position if present
        if let Some(pos) = imported_state.calculator_position {
            self.calculator.set_position(pos.0, pos.1);
        }
        
        // Select first note if any exist
        if !self.notes.is_empty() {
            self.selected_note_id = Some(self.notes[0].id.clone());
            self.note_title_input = self.notes[0].title.clone();
            use iced::widget::text_editor;
            self.editor_content = text_editor::Content::with_text(&self.notes[0].content);
        } else {
            self.selected_note_id = None;
            self.note_title_input.clear();
            use iced::widget::text_editor;
            self.editor_content = text_editor::Content::new();
        }
        
        // Clear error on success
        self.import_error = None;
        
        // Save the imported state
        self.auto_save()
    }

    pub fn handle_clear_all_data(&mut self) {
        self.show_clear_confirm = true;
    }

    pub fn handle_confirm_clear_all_data(&mut self) -> Task<Message> {
        use iced::widget::text_editor;
        
        // Log data clear
        if let Some(session) = &self.session {
            let audit_entry = AuditEntry::new(
                session.user_id.clone(),
                session.username.clone(),
                AuditAction::DataCleared,
                "data".to_string(),
                None,
                format!(
                    "Cleared {} items and {} notes",
                    self.items.len(),
                    self.notes.len()
                ),
            );
            self.audit_log.add_entry(audit_entry);
        }
        
        self.items.clear();
        self.notes.clear();
        self.selected_note_id = None;
        self.note_title_input.clear();
        self.editor_content = text_editor::Content::new();
        self.show_clear_confirm = false;
        self.auto_save()
    }

    pub fn handle_cancel_clear_all_data(&mut self) {
        self.show_clear_confirm = false;
    }

    pub fn handle_check_for_updates(&self) -> Task<Message> {
        let update_checker = self.update_checker.clone();
        Task::perform(
            async move {
                update_checker.check_for_updates().await
            },
            Message::UpdateCheckComplete,
        )
    }

    pub fn handle_update_check_complete(&mut self, result: Result<Option<crate::update_checker::UpdateInfo>, String>) {
        match result {
            Ok(Some(update_info)) => {
                self.latest_version = Some(update_info);
                self.show_update_notification = true;
            }
            Ok(None) => {
                // No update available
                self.latest_version = None;
            }
            Err(e) => {
                eprintln!("Update check failed: {}", e);
                self.latest_version = None;
            }
        }
    }

    pub fn handle_download_update(&self, download_url: String) -> Task<Message> {
        let update_checker = self.update_checker.clone();
        Task::perform(
            async move {
                update_checker.download_installer(&download_url).await
            },
            |result| match result {
                Ok(path) => Message::InstallUpdate(path),
                Err(e) => {
                    eprintln!("Download failed: {}", e);
                    Message::CloseUpdateNotification
                }
            },
        )
    }

    pub fn handle_install_update(&mut self, path: std::path::PathBuf) {
        if let Err(e) = crate::update_checker::UpdateChecker::open_installer(&path) {
            eprintln!("Failed to open installer: {}", e);
        }
        self.show_update_notification = false;
    }

    pub fn handle_close_update_notification(&mut self) {
        self.show_update_notification = false;
    }
}

