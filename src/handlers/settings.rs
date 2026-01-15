use iced::Task;
use std::collections::HashSet;
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
            sidebar_collapsed: self.sidebar_collapsed,
            show_alerts_panel: self.show_alerts_panel,
            show_search_panel: self.show_search_panel,
            current_view: self.current_view.clone(),
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
                // Provide more helpful error messages
                let error_msg = if e.to_string().contains("password_hash") {
                    format!("Import error: The file format is incompatible. Missing required field 'password_hash'. This may be an old export format. Please export a new file from the current version and try again.")
                } else {
                    format!("Invalid JSON format: {}. Please ensure the file is a valid export from this application.", e)
                };
                self.import_error = Some(error_msg);
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
        
        // Merge imported data with existing data
        // Items: Add items that don't exist (check by ID and SKU to avoid duplicates)
        let existing_item_ids: HashSet<String> = 
            self.items.iter().map(|item| item.id.clone()).collect();
        let existing_skus: HashSet<String> = 
            self.items.iter().map(|item| item.sku.to_lowercase()).collect();
        
        for imported_item in imported_state.items {
            // Skip if item with same ID already exists
            if existing_item_ids.contains(&imported_item.id) {
                continue;
            }
            // Skip if item with same SKU already exists (case-insensitive)
            if existing_skus.contains(&imported_item.sku.to_lowercase()) {
                continue;
            }
            // Add the new item
            self.items.push(imported_item);
        }
        self.filtered_items = self.search_filter.apply(&self.items);
        
        // Notes: Add notes that don't exist (check by ID)
        let existing_note_ids: HashSet<String> = 
            self.notes.iter().map(|note| note.id.clone()).collect();
        
        for imported_note in imported_state.notes {
            if !existing_note_ids.contains(&imported_note.id) {
                self.notes.push(imported_note);
            }
        }
        
        // Settings: Replace with imported settings (user likely wants imported preferences)
        self.settings = imported_state.settings.clone();
        
        // Auth store: Don't merge users from import (security - passwords aren't serialized)
        // Keep existing users, but ensure default admin exists
        self.auth_store.ensure_default_admin();
        
        // Audit log: Merge entries (append imported entries)
        // Check existing entry IDs to avoid duplicates
        let existing_audit_ids: HashSet<String> = 
            self.audit_log.get_entries().iter().map(|e| e.id.clone()).collect();
        
        for entry in imported_state.audit_log.get_entries() {
            // Only add if entry ID doesn't already exist
            if !existing_audit_ids.contains(&entry.id) {
                self.audit_log.add_entry(entry.clone());
            }
        }
        
        // Alert manager: Replace settings, then regenerate alerts from merged inventory
        *self.alert_manager.settings_mut() = imported_state.alert_manager.settings().clone();
        self.alert_manager.update_from_inventory(&self.items);
        
        // UI state: Keep current state (don't import UI preferences)
        // self.sidebar_collapsed stays as is
        // self.show_alerts_panel stays as is
        // self.show_search_panel stays as is
        // self.current_view stays as is
        
        // Update settings inputs
        self.settings_interval_input = self.settings.auto_save_interval.to_string();
        self.settings_category_input = self.settings.default_category.clone();
        
        // Update calculator position if present
        if let Some(pos) = imported_state.calculator_position {
            self.calculator.set_position(pos.0, pos.1);
        }
        
        // Update note selection: keep current selection if it still exists, otherwise select first note
        use iced::widget::text_editor;
        if let Some(ref current_note_id) = self.selected_note_id {
            // Check if current note still exists after merge
            if self.notes.iter().any(|n| n.id == *current_note_id) {
                // Current note still exists, update editor content
                if let Some(note) = self.notes.iter().find(|n| n.id == *current_note_id) {
                    self.note_title_input = note.title.clone();
                    self.editor_content = text_editor::Content::with_text(&note.content);
                }
            } else if !self.notes.is_empty() {
                // Current note doesn't exist, select first note
                self.selected_note_id = Some(self.notes[0].id.clone());
                self.note_title_input = self.notes[0].title.clone();
                self.editor_content = text_editor::Content::with_text(&self.notes[0].content);
            } else {
                // No notes exist
                self.selected_note_id = None;
                self.note_title_input.clear();
                self.editor_content = text_editor::Content::new();
            }
        } else if !self.notes.is_empty() {
            // No note was selected, select first note
            self.selected_note_id = Some(self.notes[0].id.clone());
            self.note_title_input = self.notes[0].title.clone();
            self.editor_content = text_editor::Content::with_text(&self.notes[0].content);
        } else {
            // No notes exist
            self.selected_note_id = None;
            self.note_title_input.clear();
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

    pub fn handle_check_for_updates(&mut self) -> Task<Message> {
        self.checking_for_updates = true;
        self.update_message = Some("Checking for updates...".to_string());
        let update_checker = self.update_checker.clone();
        Task::perform(
            async move {
                update_checker.check_for_updates().await
            },
            Message::UpdateCheckComplete,
        )
    }

    pub fn handle_update_check_complete(&mut self, result: Result<Option<crate::update_checker::UpdateInfo>, String>) {
        self.checking_for_updates = false;
        match result {
            Ok(Some(update_info)) => {
                let version = update_info.version.clone();
                self.latest_version = Some(update_info);
                self.show_update_notification = true;
                self.update_message = Some(format!("Update available: v{}", version));
            }
            Ok(None) => {
                // No update available
                self.latest_version = None;
                self.update_message = Some("You are running the latest version.".to_string());
            }
            Err(e) => {
                eprintln!("Update check failed: {}", e);
                self.latest_version = None;
                self.update_message = Some(format!("Failed to check for updates: {}", e));
            }
        }
    }

    pub fn handle_download_update(&mut self, download_url: String) -> Task<Message> {
        self.downloading_update = true;
        self.update_message = Some("Downloading update...".to_string());
        let update_checker = self.update_checker.clone();
        Task::perform(
            async move {
                update_checker.download_installer(&download_url).await
            },
            |result| match result {
                Ok(path) => Message::InstallUpdate(path),
                Err(e) => {
                    eprintln!("Download failed: {}", e);
                    Message::UpdateDownloadFailed(e)
                }
            },
        )
    }

    pub fn handle_update_download_failed(&mut self, error: String) {
        self.downloading_update = false;
        self.update_message = Some(format!("Download failed: {}", error));
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

