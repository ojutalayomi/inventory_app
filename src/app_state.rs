use iced::keyboard::{self, Key};
use iced::mouse;
use iced::widget::{markdown, text_editor};
use iced::{Element, Subscription, Task};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::alerts::AlertManager;
use crate::audit::AuditLog;
use crate::auth::AuthStore;
use crate::calculator::Calculator;
use crate::inventory::InventoryItem;
use crate::messages::{AppSettings, ItemDialogMode, LoadError, Message, SavedState, View};
use crate::note::Note;
use crate::persistence;
use crate::search::SearchFilter;
use crate::update_checker;
use crate::user::Session;

pub enum AppState {
    Loading,
    Login,
    Loaded,
}

pub struct InventoryApp {
    // App state
    pub state: AppState,
    pub show_about: bool,
    pub show_clear_confirm: bool,

    // Authentication state
    pub auth_store: AuthStore,
    pub session: Option<Session>,
    pub username_input: String,
    pub password_input: String,
    pub login_error: Option<String>,
    pub logging_in: bool,
    
    // User management state
    pub new_username_input: String,
    pub new_password_input: String,
    pub new_role_input: Option<crate::user::UserRole>,
    pub user_operation_error: Option<String>,

    // Audit log state
    pub audit_log: AuditLog,

    // Alert system state
    pub alert_manager: AlertManager,
    pub show_alerts_panel: bool,
    pub notification_timestamps: HashMap<String, Instant>,

    // Inventory state
    pub items: Vec<InventoryItem>,
    pub filtered_items: Vec<InventoryItem>,
    pub item_dialog_mode: Option<ItemDialogMode>,
    pub search_filter: SearchFilter,
    pub show_search_panel: bool,

    // Item dialog inputs
    pub name_input: String,
    pub sku_input: String,
    pub category_input: String,
    pub supplier_input: String,
    pub description_input: String,
    pub quantity_input: String,
    pub price_input: String,
    pub item_validation_error: Option<String>,
    pub similar_items_warning: Vec<String>,

    // Editor/Notes state
    pub notes: Vec<Note>,
    pub selected_note_id: Option<String>,
    pub note_title_input: String,
    pub editor_content: text_editor::Content,
    pub delete_note_confirm: Option<String>,

    // Calculator state
    pub calculator: Calculator,

    // Settings state
    pub settings: AppSettings,
    pub settings_interval_input: String,
    pub settings_category_input: String,
    pub settings_notification_throttle_input: String,
    pub import_file_picker_open: bool,
    pub import_error: Option<String>,

    // Update state
    pub update_checker: update_checker::UpdateChecker,
    pub latest_version: Option<update_checker::UpdateInfo>,
    pub update_release_notes_items: Option<Vec<markdown::Item>>,
    pub show_update_notification: bool,
    pub update_download_progress: Option<f32>,
    pub checking_for_updates: bool,
    pub downloading_update: bool,
    pub update_message: Option<String>,
    
    // View state
    pub current_view: View,
    
    // Sidebar state
    pub sidebar_collapsed: bool,
}

impl InventoryApp {
    pub fn theme(&self) -> iced::Theme {
        match self.settings.theme {
            crate::messages::AppTheme::Dark => iced::Theme::Dark,
            crate::messages::AppTheme::Light => iced::Theme::Light,
        }
    }

    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: AppState::Loading,
                show_about: false,
                show_clear_confirm: false,
                auth_store: AuthStore::new(),
                session: None,
                username_input: String::new(),
                password_input: String::new(),
                login_error: None,
                logging_in: false,
                new_username_input: String::new(),
                new_password_input: String::new(),
                new_role_input: None,
                user_operation_error: None,
                audit_log: AuditLog::new(),
                alert_manager: AlertManager::new(),
                show_alerts_panel: false,
                notification_timestamps: HashMap::new(),
                items: Vec::new(),
                filtered_items: Vec::new(),
                item_dialog_mode: None,
                search_filter: SearchFilter::new(),
                show_search_panel: false,
                name_input: String::new(),
                sku_input: String::new(),
                category_input: String::new(),
                supplier_input: String::new(),
                description_input: String::new(),
                quantity_input: String::new(),
                price_input: String::new(),
                item_validation_error: None,
                similar_items_warning: Vec::new(),
                notes: Vec::new(),
                selected_note_id: None,
                note_title_input: String::new(),
                editor_content: text_editor::Content::new(),
                delete_note_confirm: None,
                calculator: Calculator::new(),
                settings: AppSettings::default(),
                settings_interval_input: String::from("5"),
                settings_category_input: String::from("General"),
                settings_notification_throttle_input: String::from("30"),
                import_file_picker_open: false,
                import_error: None,
                update_checker: update_checker::UpdateChecker::new(
                    "ojutalayomi".to_string(),
                    "inventory_app".to_string(),
                ),
                latest_version: None,
                update_release_notes_items: None,
                show_update_notification: false,
                update_download_progress: None,
                checking_for_updates: false,
                downloading_update: false,
                update_message: None,
                current_view: View::Inventory,
                sidebar_collapsed: false,
            },
            Task::perform(persistence::load_state(), Message::Loaded),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(Ok(state)) => self.handle_loaded_success(state),
            Message::Loaded(Err(LoadError::FileNotFound)) => self.handle_loaded_file_not_found(),
            Message::Loaded(Err(LoadError::FormatError)) => self.handle_loaded_format_error(),

            // Authentication Messages
            Message::UsernameChanged(value) => {
                self.handle_username_changed(value);
                Task::none()
            }
            Message::PasswordChanged(value) => {
                self.handle_password_changed(value);
                Task::none()
            }
            Message::AttemptLogin => self.handle_attempt_login(),
            Message::LoginSuccess => {
                self.handle_login_success();
                Task::none()
            }
            Message::LoginFailed(error) => {
                self.handle_login_failed(error);
                Task::none()
            }
            Message::Logout => self.handle_logout(),

            // User Management Messages
            Message::NewUsernameChanged(value) => {
                self.handle_new_username_changed(value);
                Task::none()
            }
            Message::NewPasswordChanged(value) => {
                self.handle_new_password_changed(value);
                Task::none()
            }
            Message::NewRoleChanged(role) => {
                self.handle_new_role_changed(role);
                Task::none()
            }
            Message::CreateUser => self.handle_create_user(),
            Message::EditUser(user_id) => {
                self.handle_edit_user(user_id);
                Task::none()
            }
            Message::DeleteUser(user_id) => self.handle_delete_user(user_id),
            Message::UserOperationResult(result) => {
                self.handle_user_operation_result(result);
                Task::none()
            }

            // Search and Filter Messages
            Message::ToggleSearchPanel => self.handle_toggle_search_panel(),
            Message::SearchQueryChanged(query) => {
                self.handle_search_query_changed(query);
                Task::none()
            }
            Message::CategoryFilterChanged(category) => {
                self.handle_category_filter_changed(category);
                Task::none()
            }
            Message::SupplierFilterChanged(supplier) => {
                self.handle_supplier_filter_changed(supplier);
                Task::none()
            }
            Message::MinQuantityChanged(value) => {
                self.handle_min_quantity_changed(value);
                Task::none()
            }
            Message::MaxQuantityChanged(value) => {
                self.handle_max_quantity_changed(value);
                Task::none()
            }
            Message::MinPriceChanged(value) => {
                self.handle_min_price_changed(value);
                Task::none()
            }
            Message::MaxPriceChanged(value) => {
                self.handle_max_price_changed(value);
                Task::none()
            }
            Message::SortFieldChanged(field) => {
                self.handle_sort_field_changed(field);
                Task::none()
            }
            Message::SortDirectionToggled => {
                self.handle_sort_direction_toggled();
                Task::none()
            }
            Message::ClearFilters => {
                self.handle_clear_filters();
                Task::none()
            }

            // Alert Messages
            Message::ToggleAlertsPanel => self.handle_toggle_alerts_panel(),
            Message::AcknowledgeAlert(alert_id) => self.handle_acknowledge_alert(alert_id),
            Message::AcknowledgeAllAlerts => self.handle_acknowledge_all_alerts(),
            Message::ClearAcknowledgedAlerts => self.handle_clear_acknowledged_alerts(),
            Message::AlertLowStockThresholdChanged(value) => self.handle_alert_low_stock_threshold_changed(value),
            Message::AlertCriticalThresholdChanged(value) => self.handle_alert_critical_threshold_changed(value),
            Message::ToggleAlertsEnabled => self.handle_toggle_alerts_enabled(),
            Message::ToggleAlertNotifications => self.handle_toggle_alert_notifications(),
            Message::UpdateAlertSettings => self.handle_update_alert_settings(),

            // Audit Log Messages
            Message::ExportAuditLog => self.handle_export_audit_log(),

            // Item Dialog Messages
            Message::OpenAddDialog => {
                self.handle_open_add_dialog();
                Task::none()
            }
            Message::OpenEditDialog(item_id) => {
                self.handle_open_edit_dialog(item_id);
                Task::none()
            }
            Message::CloseItemDialog => {
                self.handle_close_item_dialog();
                Task::none()
            }
            Message::NameChanged(value) => {
                self.handle_name_changed(value);
                Task::none()
            }
            Message::SkuChanged(value) => {
                self.handle_sku_changed(value);
                Task::none()
            }
            Message::CategoryChanged(value) => {
                self.handle_category_changed(value);
                Task::none()
            }
            Message::SupplierChanged(value) => {
                self.handle_supplier_changed(value);
                Task::none()
            }
            Message::DescriptionChanged(value) => {
                self.handle_description_changed(value);
                Task::none()
            }
            Message::QuantityChanged(value) => {
                self.handle_quantity_changed(value);
                Task::none()
            }
            Message::PriceChanged(value) => {
                self.handle_price_changed(value);
                Task::none()
            }
            Message::SubmitItem => self.handle_submit_item(),
            Message::DeleteItem(item_id) => self.handle_delete_item(item_id),
            Message::ExportInventoryCsv => self.handle_export_inventory_csv(),
            Message::InventoryViewModeChanged(mode) => {
                self.handle_inventory_view_mode_changed(mode)
            }

            // Notes Messages
            Message::CreateNote => self.handle_create_note(),
            Message::SelectNote(note_id) => {
                self.handle_select_note(note_id);
                Task::none()
            }
            Message::UpdateNoteTitle(title) => self.handle_update_note_title(title),
            Message::UpdateNoteContent(action) => self.handle_update_note_content(action),
            Message::DeleteNote(note_id) => {
                self.handle_delete_note(note_id);
                Task::none()
            }
            Message::ConfirmDeleteNote => self.handle_confirm_delete_note(),
            Message::CloseDeleteConfirm => {
                self.handle_close_delete_confirm();
                Task::none()
            }
            Message::ExportNote(format) => self.handle_export_note(format),

            // Calculator Messages
            Message::ToggleCalculator => {
                self.handle_toggle_calculator();
                Task::none()
            }
            Message::CalculatorInput(digit) => {
                self.handle_calculator_input(digit);
                Task::none()
            }
            Message::CalculatorOperation(op) => {
                self.handle_calculator_operation(op);
                Task::none()
            }
            Message::CalculatorEquals => {
                self.handle_calculator_equals();
                Task::none()
            }
            Message::CalculatorClear => {
                self.handle_calculator_clear();
                Task::none()
            }
            Message::CalculatorDragStart => {
                self.handle_calculator_drag_start();
                Task::none()
            }
            Message::CalculatorDragMove(x, y) => {
                self.handle_calculator_drag_move(x, y);
                Task::none()
            }
            Message::CalculatorDragEnd => self.handle_calculator_drag_end(),

            // View Switching
            Message::SwitchView(view) => {
                self.current_view = view;
                Task::none()
            }

            // App Actions
            Message::Save => self.auto_save(),
            Message::ShowAbout => {
                self.show_about = true;
                Task::none()
            }
            Message::CloseAbout => {
                self.show_about = false;
                Task::none()
            }

            // Settings Messages
            Message::ToggleAutoSave => self.handle_toggle_auto_save(),
            Message::AutoSaveIntervalChanged(value) => self.handle_auto_save_interval_changed(value),
            Message::DefaultCategoryChanged(value) => self.handle_default_category_changed(value),
            Message::CurrencyChanged(value) => self.handle_currency_changed(value),
            Message::ThemeChanged(theme) => self.handle_theme_changed(theme),
            Message::ToggleLoadingScreen => self.handle_toggle_loading_screen(),
            Message::LayoutStyleChanged(style) => self.handle_layout_style_changed(style),
            Message::ToggleDeviceNotifications => self.handle_toggle_device_notifications(),
            Message::ToggleUpdateNotifications => self.handle_toggle_update_notifications(),
            Message::NotificationThrottleChanged(value) => {
                self.handle_notification_throttle_changed(value)
            }
            Message::ToggleSidebar => {
                self.sidebar_collapsed = !self.sidebar_collapsed;
                self.auto_save()
            }
            Message::ExportData => self.handle_export_data(),
            Message::ImportData => self.handle_import_data(),
            Message::OpenImportFilePicker => self.handle_open_import_file_picker(),
            Message::ImportFileSelected(path) => self.handle_import_file_selected(path),
            Message::ClearAllData => {
                self.handle_clear_all_data();
                Task::none()
            }
            Message::ConfirmClearAllData => self.handle_confirm_clear_all_data(),
            Message::CancelClearAllData => {
                self.handle_cancel_clear_all_data();
                Task::none()
            }

            // Update messages
            Message::CheckForUpdates => self.handle_check_for_updates(),
            Message::UpdateCheckComplete(result) => {
                self.handle_update_check_complete(result);
                Task::none()
            }
            Message::DownloadUpdate(download_url) => self.handle_download_update(download_url),
            Message::InstallUpdate(path) => {
                self.handle_install_update(path);
                Task::none()
            }
            Message::UpdateDownloadFailed(error) => {
                self.handle_update_download_failed(error);
                Task::none()
            }
            Message::CloseUpdateNotification => {
                self.handle_close_update_notification();
                Task::none()
            }
        }
    }

    fn handle_loaded_success(&mut self, state: SavedState) -> Task<Message> {
        self.items = state.items;
        self.filtered_items = self.search_filter.apply(&self.items);
        self.alert_manager = state.alert_manager;
        let _ = self.alert_manager.update_from_inventory(&self.items);
        self.notes = state.notes;
        self.settings = state.settings;
        self.auth_store = state.auth_store;
        // Ensure default admin user exists with valid password hash
        // This is needed because password_hash is not serialized for security
        self.auth_store.ensure_default_admin();
        self.audit_log = state.audit_log;
        self.sidebar_collapsed = state.sidebar_collapsed;
        self.show_alerts_panel = state.show_alerts_panel;
        self.show_search_panel = state.show_search_panel;
        self.current_view = state.current_view;
        self.settings_interval_input = self.settings.auto_save_interval.to_string();
        self.settings_category_input = self.settings.default_category.clone();
        self.settings_notification_throttle_input =
            self.settings.notification_throttle_seconds.to_string();
        if let Some(pos) = state.calculator_position {
            self.calculator.set_position(pos.0, pos.1);
        }
        // Select first note if any exist
        if !self.notes.is_empty() {
            self.selected_note_id = Some(self.notes[0].id.clone());
            self.note_title_input = self.notes[0].title.clone();
            self.editor_content = text_editor::Content::with_text(&self.notes[0].content);
        }
        if self.settings.show_loading_screen {
            Self::delay(3);
        }
        self.state = AppState::Login;
        Task::none()
    }

    fn handle_loaded_file_not_found(&mut self) -> Task<Message> {
        // First time running, create a welcome note
        let welcome_note = Note::new("Welcome to Inventory Manager".to_string());
        self.selected_note_id = Some(welcome_note.id.clone());
        self.note_title_input = welcome_note.title.clone();
        self.notes.push(welcome_note);
        Self::delay(3);
        self.state = AppState::Login;
        Task::none()
    }

    fn handle_loaded_format_error(&mut self) -> Task<Message> {
        // Corrupted file, start fresh
        Self::delay(3);
        self.state = AppState::Login;
        Task::none()
    }

    pub fn clear_item_inputs(&mut self) {
        self.name_input.clear();
        self.sku_input.clear();
        self.category_input.clear();
        self.supplier_input.clear();
        self.description_input.clear();
        self.quantity_input.clear();
        self.price_input.clear();
        self.item_validation_error = None;
        self.similar_items_warning.clear();
    }

    pub fn auto_save(&self) -> Task<Message> {
        if !self.settings.auto_save_enabled {
            return Task::none();
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
        Task::perform(
            async move {
                persistence::save_state(&state).await.ok();
            },
            |_| Message::Save,
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let keyboard_sub = keyboard::on_key_press(|key, modifiers| {
            match (key.as_ref(), modifiers) {
                // Ctrl/Cmd + S to save
                (Key::Character("s"), keyboard::Modifiers::COMMAND)
                | (Key::Character("s"), keyboard::Modifiers::CTRL) => Some(Message::Save),
                // Ctrl/Cmd + 1 for Inventory view
                (Key::Character("1"), keyboard::Modifiers::COMMAND)
                | (Key::Character("1"), keyboard::Modifiers::CTRL) => {
                    Some(Message::SwitchView(View::Inventory))
                }
                // Ctrl/Cmd + 2 for Editor view
                (Key::Character("2"), keyboard::Modifiers::COMMAND)
                | (Key::Character("2"), keyboard::Modifiers::CTRL) => {
                    Some(Message::SwitchView(View::Editor))
                }
                // Ctrl/Cmd + K for Calculator
                (Key::Character("k"), keyboard::Modifiers::COMMAND)
                | (Key::Character("k"), keyboard::Modifiers::CTRL) => {
                    Some(Message::ToggleCalculator)
                }
                // Ctrl/Cmd + I for About
                (Key::Character("i"), keyboard::Modifiers::COMMAND)
                | (Key::Character("i"), keyboard::Modifiers::CTRL) => Some(Message::ShowAbout),
                // Ctrl/Cmd + N for New Note
                (Key::Character("n"), keyboard::Modifiers::COMMAND)
                | (Key::Character("n"), keyboard::Modifiers::CTRL) => Some(Message::CreateNote),
                // Escape to close overlays
                (Key::Named(keyboard::key::Named::Escape), _) => Some(Message::CloseAbout),
                _ => None,
            }
        });

        let mouse_sub = if self.calculator.dragging {
            iced::event::listen_with(|event, _status, _id| match event {
                iced::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    Some(Message::CalculatorDragMove(position.x, position.y))
                }
                iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    Some(Message::CalculatorDragEnd)
                }
                _ => None,
            })
        } else {
            Subscription::none()
        };

        Subscription::batch([keyboard_sub, mouse_sub])
    }

    pub fn view(&self) -> Element<Message> {
        match self.state {
            AppState::Loading => crate::views::loading::view(&self.settings.theme),
            AppState::Login => crate::views::login::view(
                &self.username_input,
                &self.password_input,
                self.login_error.as_deref(),
                &self.settings.theme,
                self.logging_in,
            ),
            AppState::Loaded => {
                let main_content = self.view_loaded();

                // Stack overlays: main content -> item dialog -> about dialog
                let mut stack = vec![main_content];

                if let Some(ref mode) = self.item_dialog_mode {
                    stack.push(crate::views::item_dialog::view(
                        mode,
                        &self.name_input,
                        &self.sku_input,
                        &self.category_input,
                        &self.supplier_input,
                        &self.description_input,
                        &self.quantity_input,
                        &self.price_input,
                        self.item_validation_error.as_deref(),
                        &self.similar_items_warning,
                        &self.settings.theme,
                    ));
                }

                if self.show_about {
                    stack.push(crate::views::about::view(&self.settings.theme));
                }

                if self.show_clear_confirm {
                    stack.push(crate::views::dialogs::view_clear_confirm(&self.settings.theme));
                }

                if self.show_update_notification && self.latest_version.is_some() {
                    let update_info = self.latest_version.as_ref().unwrap();
                    stack.push(crate::views::dialogs::view_update_notification(
                        update_info,
                        self.update_release_notes_items.as_deref(),
                        &self.settings.theme,
                    ));
                }

                if stack.len() == 1 {
                    stack.into_iter().next().unwrap()
                } else {
                    iced::widget::stack(stack).into()
                }
            }
        }
    }

    fn delay(seconds: u64) {
        std::thread::sleep(std::time::Duration::from_secs(seconds));
    }

    pub(crate) fn update_alerts_from_inventory(&mut self) {
        let new_alerts = self.alert_manager.update_from_inventory(&self.items);
        self.notify_new_alerts(&new_alerts);
    }

    pub(crate) fn maybe_send_device_notification(
        &mut self,
        key: &str,
        title: &str,
        body: &str,
    ) {
        let throttle_seconds = self
            .settings
            .notification_throttle_seconds
            .max(1)
            .min(86_400);
        let throttle_window = Duration::from_secs(throttle_seconds as u64);
        let now = Instant::now();

        if let Some(last_sent) = self.notification_timestamps.get(key) {
            if now.duration_since(*last_sent) < throttle_window {
                return;
            }
        }

        self.notification_timestamps
            .insert(key.to_string(), now);
        crate::notifications::send_notification(title, body);
    }

    fn notify_new_alerts(&mut self, alerts: &[crate::alerts::StockAlert]) {
        if !self.settings.device_notifications_enabled
            || !self.alert_manager.settings().show_notifications
        {
            return;
        }

        for alert in alerts {
            let key = format!("alert:{}", alert.id);
            let title = format!("{}: {}", alert.alert_type, alert.item_name);
            let body = format!(
                "SKU: {} | Qty: {}",
                alert.item_sku, alert.current_quantity
            );
            self.maybe_send_device_notification(&key, &title, &body);
        }
    }
}

