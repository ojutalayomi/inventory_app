use iced::keyboard::{self, Key};
use iced::mouse;
use iced::widget::{button, column, container, row, text, text_editor};
use iced::window;
use iced::{Element, Length, Subscription, Task};

mod alerts;
mod audit;
mod auth;
mod calculator;
mod errors;
mod inventory;
mod messages;
mod note;
mod persistence;
mod search;
mod user;
mod views;

use alerts::AlertManager;
use audit::{AuditAction, AuditEntry, AuditLog};
use auth::AuthStore;
use calculator::Calculator;
use inventory::InventoryItem;
use messages::{AppSettings, ItemDialogMode, LoadError, Message, SavedState, View};
use note::Note;
use search::SearchFilter;
use user::{Session, UserRole};

fn main() -> iced::Result {
    iced::application(
        "Inventory Manager",
        InventoryApp::update,
        InventoryApp::view,
    )
    .theme(InventoryApp::theme)
    .subscription(InventoryApp::subscription)
    .window(window::Settings {
        size: iced::Size::new(1200.0, 800.0),
        min_size: Some(iced::Size::new(900.0, 600.0)),
        ..Default::default()
    })
    .run_with(InventoryApp::new)
}

enum AppState {
    Loading,
    Login,
    Loaded,
}

struct InventoryApp {
    // App state
    state: AppState,
    show_about: bool,
    show_clear_confirm: bool,

    // Authentication state
    auth_store: AuthStore,
    session: Option<Session>,
    username_input: String,
    password_input: String,
    login_error: Option<String>,

    // User management state
    new_username_input: String,
    new_password_input: String,
    new_role_input: Option<UserRole>,
    user_operation_error: Option<String>,

    // Audit log state
    audit_log: AuditLog,

    // Alert system state
    alert_manager: AlertManager,
    show_alerts_panel: bool,

    // Inventory state
    items: Vec<InventoryItem>,
    filtered_items: Vec<InventoryItem>,
    item_dialog_mode: Option<ItemDialogMode>,
    search_filter: SearchFilter,
    show_search_panel: bool,

    // Item dialog inputs
    name_input: String,
    sku_input: String,
    category_input: String,
    supplier_input: String,
    description_input: String,
    quantity_input: String,
    price_input: String,
    item_validation_error: Option<String>,
    similar_items_warning: Vec<String>,

    // Editor/Notes state
    notes: Vec<Note>,
    selected_note_id: Option<String>,
    note_title_input: String,
    editor_content: text_editor::Content,
    delete_note_confirm: Option<String>,

    // Calculator state
    calculator: Calculator,

    // Settings state
    settings: AppSettings,
    settings_interval_input: String,
    settings_category_input: String,

    // View state
    current_view: View,
}

impl InventoryApp {
    fn theme(&self) -> iced::Theme {
        match self.settings.theme {
            messages::AppTheme::Dark => iced::Theme::Dark,
            messages::AppTheme::Light => iced::Theme::Light,
        }
    }

    fn new() -> (Self, Task<Message>) {
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
                new_username_input: String::new(),
                new_password_input: String::new(),
                new_role_input: None,
                user_operation_error: None,
                audit_log: AuditLog::new(),
                alert_manager: AlertManager::new(),
                show_alerts_panel: false,
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
                current_view: View::Inventory,
            },
            Task::perform(persistence::load_state(), Message::Loaded),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(Ok(state)) => {
                self.items = state.items;
                self.filtered_items = self.search_filter.apply(&self.items);
                self.alert_manager = state.alert_manager;
                self.alert_manager.update_from_inventory(&self.items);
                self.notes = state.notes;
                self.settings = state.settings;
                self.auth_store = state.auth_store;
                self.audit_log = state.audit_log;
                self.settings_interval_input = self.settings.auto_save_interval.to_string();
                self.settings_category_input = self.settings.default_category.clone();
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
            }
            Message::Loaded(Err(LoadError::FileNotFound)) => {
                // First time running, create a welcome note
                let welcome_note = Note::new("Welcome to Inventory Manager".to_string());
                self.selected_note_id = Some(welcome_note.id.clone());
                self.note_title_input = welcome_note.title.clone();
                self.notes.push(welcome_note);
                Self::delay(3);
                self.state = AppState::Login;
            }
            Message::Loaded(Err(LoadError::FormatError)) => {
                // Corrupted file, start fresh
                Self::delay(3);
                self.state = AppState::Login;
            }

            // Authentication Messages
            Message::UsernameChanged(value) => {
                self.username_input = value;
                self.login_error = None;
            }
            Message::PasswordChanged(value) => {
                self.password_input = value;
                self.login_error = None;
            }
            Message::AttemptLogin => {
                if let Some(session) = self
                    .auth_store
                    .authenticate(&self.username_input, &self.password_input)
                {
                    // Log successful login
                    let audit_entry = AuditEntry::new(
                        session.user_id.clone(),
                        session.username.clone(),
                        AuditAction::UserLogin,
                        "user".to_string(),
                        Some(session.user_id.clone()),
                        "User logged in successfully".to_string(),
                    );
                    self.audit_log.add_entry(audit_entry);

                    self.session = Some(session);
                    self.username_input.clear();
                    self.password_input.clear();
                    self.login_error = None;
                    self.state = AppState::Loaded;
                    return self.auto_save();
                } else {
                    self.login_error = Some("Invalid username or password".to_string());
                }
            }
            Message::LoginSuccess => {
                self.state = AppState::Loaded;
            }
            Message::LoginFailed(error) => {
                self.login_error = Some(error);
            }
            Message::Logout => {
                if let Some(session) = &self.session {
                    // Log logout
                    let audit_entry = AuditEntry::new(
                        session.user_id.clone(),
                        session.username.clone(),
                        AuditAction::UserLogout,
                        "user".to_string(),
                        Some(session.user_id.clone()),
                        "User logged out".to_string(),
                    );
                    self.audit_log.add_entry(audit_entry);
                }
                self.session = None;
                self.state = AppState::Login;
                self.current_view = View::Inventory;
                return self.auto_save();
            }

            // User Management Messages
            Message::NewUsernameChanged(value) => {
                self.new_username_input = value;
                self.user_operation_error = None;
            }
            Message::NewPasswordChanged(value) => {
                self.new_password_input = value;
                self.user_operation_error = None;
            }
            Message::NewRoleChanged(role) => {
                self.new_role_input = Some(role);
                self.user_operation_error = None;
            }
            Message::CreateUser => {
                if let Some(role) = self.new_role_input {
                    if !self.new_username_input.is_empty() && !self.new_password_input.is_empty() {
                        match self.auth_store.add_user(
                            self.new_username_input.clone(),
                            &self.new_password_input,
                            role,
                        ) {
                            Ok(user) => {
                                // Log user creation
                                if let Some(session) = &self.session {
                                    let audit_entry = AuditEntry::new(
                                        session.user_id.clone(),
                                        session.username.clone(),
                                        AuditAction::UserCreated,
                                        "user".to_string(),
                                        Some(user.id.clone()),
                                        format!(
                                            "Created user: {} with role: {:?}",
                                            user.username, user.role
                                        ),
                                    );
                                    self.audit_log.add_entry(audit_entry);
                                }

                                self.new_username_input.clear();
                                self.new_password_input.clear();
                                self.new_role_input = None;
                                self.user_operation_error = None;
                                return self.auto_save();
                            }
                            Err(e) => {
                                self.user_operation_error = Some(e);
                            }
                        }
                    } else {
                        self.user_operation_error =
                            Some("Username and password are required".to_string());
                    }
                } else {
                    self.user_operation_error = Some("Please select a role".to_string());
                }
            }
            Message::EditUser(_user_id) => {
                // TODO: Implement user editing dialog
                self.user_operation_error = Some("User editing coming soon".to_string());
            }
            Message::DeleteUser(user_id) => {
                let username = self
                    .auth_store
                    .get_user(&user_id)
                    .map(|u| u.username.clone());

                match self.auth_store.delete_user(&user_id) {
                    Ok(_) => {
                        // Log user deletion
                        if let Some(session) = &self.session {
                            if let Some(uname) = username {
                                let audit_entry = AuditEntry::new(
                                    session.user_id.clone(),
                                    session.username.clone(),
                                    AuditAction::UserDeleted,
                                    "user".to_string(),
                                    Some(user_id.clone()),
                                    format!("Deleted user: {}", uname),
                                );
                                self.audit_log.add_entry(audit_entry);
                            }
                        }

                        self.user_operation_error = None;
                        return self.auto_save();
                    }
                    Err(e) => {
                        self.user_operation_error = Some(e);
                    }
                }
            }
            Message::UserOperationResult(result) => {
                if let Err(e) = result {
                    self.user_operation_error = Some(e);
                } else {
                    self.user_operation_error = None;
                }
            }

            // Search and Filter Messages
            Message::ToggleSearchPanel => {
                self.show_search_panel = !self.show_search_panel;
            }
            Message::SearchQueryChanged(query) => {
                self.search_filter.query = query;
                self.filtered_items = self.search_filter.apply(&self.items);
            }
            Message::CategoryFilterChanged(category) => {
                self.search_filter.category_filter = if category.is_empty() {
                    None
                } else {
                    Some(category)
                };
                self.filtered_items = self.search_filter.apply(&self.items);
            }
            Message::SupplierFilterChanged(supplier) => {
                self.search_filter.supplier_filter = if supplier.is_empty() {
                    None
                } else {
                    Some(supplier)
                };
                self.filtered_items = self.search_filter.apply(&self.items);
            }
            Message::MinQuantityChanged(value) => {
                self.search_filter.min_quantity = value.parse().ok();
                self.filtered_items = self.search_filter.apply(&self.items);
            }
            Message::MaxQuantityChanged(value) => {
                self.search_filter.max_quantity = value.parse().ok();
                self.filtered_items = self.search_filter.apply(&self.items);
            }
            Message::MinPriceChanged(value) => {
                self.search_filter.min_price = value.parse().ok();
                self.filtered_items = self.search_filter.apply(&self.items);
            }
            Message::MaxPriceChanged(value) => {
                self.search_filter.max_price = value.parse().ok();
                self.filtered_items = self.search_filter.apply(&self.items);
            }
            Message::SortFieldChanged(field) => {
                self.search_filter.sort_field = Some(field);
                self.filtered_items = self.search_filter.apply(&self.items);
            }
            Message::SortDirectionToggled => {
                self.search_filter.sort_direction = match self.search_filter.sort_direction {
                    search::SortDirection::Ascending => search::SortDirection::Descending,
                    search::SortDirection::Descending => search::SortDirection::Ascending,
                };
                self.filtered_items = self.search_filter.apply(&self.items);
            }
            Message::ClearFilters => {
                self.search_filter.clear();
                self.filtered_items = self.search_filter.apply(&self.items);
            }

            // Alert Messages
            Message::ToggleAlertsPanel => {
                self.show_alerts_panel = !self.show_alerts_panel;
            }
            Message::AcknowledgeAlert(alert_id) => {
                self.alert_manager.acknowledge_alert(&alert_id);
                return self.auto_save();
            }
            Message::AcknowledgeAllAlerts => {
                self.alert_manager.acknowledge_all();
                return self.auto_save();
            }
            Message::ClearAcknowledgedAlerts => {
                self.alert_manager.clear_acknowledged();
                return self.auto_save();
            }
            Message::AlertLowStockThresholdChanged(value) => {
                if let Ok(threshold) = value.parse::<u32>() {
                    self.alert_manager.settings_mut().low_stock_threshold = threshold;
                    self.alert_manager.update_from_inventory(&self.items);
                    return self.auto_save();
                }
            }
            Message::AlertCriticalThresholdChanged(value) => {
                if let Ok(threshold) = value.parse::<u32>() {
                    self.alert_manager.settings_mut().critically_low_threshold = threshold;
                    self.alert_manager.update_from_inventory(&self.items);
                    return self.auto_save();
                }
            }
            Message::ToggleAlertsEnabled => {
                self.alert_manager.settings_mut().enabled = !self.alert_manager.settings().enabled;
                self.alert_manager.update_from_inventory(&self.items);
                return self.auto_save();
            }
            Message::ToggleAlertNotifications => {
                self.alert_manager.settings_mut().show_notifications = !self.alert_manager.settings().show_notifications;
                return self.auto_save();
            }
            Message::UpdateAlertSettings => {
                self.alert_manager.update_from_inventory(&self.items);
                return self.auto_save();
            }

            // Audit Log Messages
            Message::ExportAuditLog => {
                let csv_content = self.audit_log.export_to_csv();
                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                let filename = format!("audit_log_{}.csv", timestamp);

                return Task::perform(
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
                );
            }

            // Item Dialog Messages
            Message::OpenAddDialog => {
                // Check permissions
                if let Some(session) = &self.session {
                    if session.role.can_create() {
                        self.item_dialog_mode = Some(ItemDialogMode::Add);
                        self.clear_item_inputs();
                        // Pre-fill with default category from settings
                        self.category_input = self.settings.default_category.clone();
                    }
                }
            }
            Message::OpenEditDialog(item_id) => {
                // Check permissions
                if let Some(session) = &self.session {
                    if session.role.can_edit() {
                        if let Some(item) = self.items.iter().find(|i| i.id == item_id) {
                            self.item_dialog_mode = Some(ItemDialogMode::Edit(item_id.clone()));
                            self.name_input = item.name.clone();
                            self.sku_input = item.sku.clone();
                            self.category_input = item.category.clone();
                            self.supplier_input = item.supplier.clone();
                            self.description_input = item.description.clone();
                            self.quantity_input = item.quantity.to_string();
                            self.price_input = item.price.to_string();
                        }
                    }
                }
            }
            Message::CloseItemDialog => {
                self.item_dialog_mode = None;
                self.clear_item_inputs();
            }
            Message::NameChanged(value) => {
                self.name_input = value.clone();
                self.item_validation_error = None;

                // Check for similar items as the user types
                if value.len() > 3 {
                    self.similar_items_warning = errors::find_similar_items(&value, &self.items);
                } else {
                    self.similar_items_warning.clear();
                }
            }
            Message::SkuChanged(value) => {
                self.sku_input = value;
                self.item_validation_error = None;
            }
            Message::CategoryChanged(value) => {
                self.category_input = value;
                self.item_validation_error = None;
            }
            Message::SupplierChanged(value) => {
                self.supplier_input = value;
                self.item_validation_error = None;
            }
            Message::DescriptionChanged(value) => {
                self.description_input = value;
                self.item_validation_error = None;
            }
            Message::QuantityChanged(value) => {
                self.quantity_input = value;
                self.item_validation_error = None;
            }
            Message::PriceChanged(value) => {
                self.price_input = value;
                self.item_validation_error = None;
            }
            Message::SubmitItem => {
                // Validate all fields
                use errors::*;

                // Validate name
                if let Err(e) = validate_required("Name", &self.name_input) {
                    self.item_validation_error = Some(e.to_string());
                    return Task::none();
                }
                if let Err(e) = validate_length("Name", &self.name_input, 1, 200) {
                    self.item_validation_error = Some(e.to_string());
                    return Task::none();
                }

                // Validate SKU
                if let Err(e) = validate_sku_format(&self.sku_input) {
                    self.item_validation_error = Some(e.to_string());
                    return Task::none();
                }

                // Check for duplicate SKU
                let exclude_id = if let Some(ItemDialogMode::Edit(id)) = &self.item_dialog_mode {
                    Some(id.as_str())
                } else {
                    None
                };

                if let Err(e) = check_duplicate_sku(&self.sku_input, &self.items, exclude_id) {
                    self.item_validation_error = Some(e.to_string());
                    return Task::none();
                }

                // Validate quantity
                let quantity = match validate_quantity(&self.quantity_input) {
                    Ok(q) => q,
                    Err(e) => {
                        self.item_validation_error = Some(e.to_string());
                        return Task::none();
                    }
                };

                // Validate price
                let price = match validate_price(&self.price_input) {
                    Ok(p) => p,
                    Err(e) => {
                        self.item_validation_error = Some(e.to_string());
                        return Task::none();
                    }
                };

                // All validations passed
                match &self.item_dialog_mode {
                    Some(ItemDialogMode::Add) => {
                        let new_item = InventoryItem::new(
                            self.name_input.clone(),
                            self.sku_input.clone(),
                            self.category_input.clone(),
                            self.supplier_input.clone(),
                            self.description_input.clone(),
                            quantity,
                            price,
                        );

                        // Log item creation
                        if let Some(session) = &self.session {
                            let audit_entry = AuditEntry::new(
                                session.user_id.clone(),
                                session.username.clone(),
                                AuditAction::ItemCreated,
                                "item".to_string(),
                                Some(new_item.id.clone()),
                                format!("Created item: {} (SKU: {})", new_item.name, new_item.sku),
                            );
                            self.audit_log.add_entry(audit_entry);
                        }

                                self.items.push(new_item);
                                self.filtered_items = self.search_filter.apply(&self.items);
                                self.alert_manager.update_from_inventory(&self.items);
                            }
                    Some(ItemDialogMode::Edit(item_id)) => {
                        if let Some(item) = self.items.iter_mut().find(|i| i.id == *item_id) {
                            let old_values = format!(
                                "{} | {} | Qty: {} | ${:.2}",
                                item.name, item.sku, item.quantity, item.price
                            );

                            item.name = self.name_input.clone();
                            item.sku = self.sku_input.clone();
                            item.category = self.category_input.clone();
                            item.supplier = self.supplier_input.clone();
                            item.description = self.description_input.clone();
                            item.quantity = quantity;
                            item.price = price;
                            item.update_timestamp();

                            let new_values = format!(
                                "{} | {} | Qty: {} | ${:.2}",
                                item.name, item.sku, item.quantity, item.price
                            );

                            // Log item update
                            if let Some(session) = &self.session {
                                let audit_entry = AuditEntry::new(
                                    session.user_id.clone(),
                                    session.username.clone(),
                                    AuditAction::ItemUpdated,
                                    "item".to_string(),
                                    Some(item_id.clone()),
                                    format!("Updated item: {}", item.name),
                                )
                                .with_values(Some(old_values), Some(new_values));
                                self.audit_log.add_entry(audit_entry);
                            }
                        }
                    }
                    None => {}
                }
                self.item_dialog_mode = None;
                self.clear_item_inputs();
                self.item_validation_error = None;
                self.filtered_items = self.search_filter.apply(&self.items);
                self.alert_manager.update_from_inventory(&self.items);
                return self.auto_save();
            }
            Message::DeleteItem(item_id) => {
                // Check permissions
                if let Some(session) = &self.session {
                    if session.role.can_delete() {
                        let deleted_item = self
                            .items
                            .iter()
                            .find(|i| i.id == item_id)
                            .map(|i| format!("{} (SKU: {})", i.name, i.sku));

                        self.items.retain(|item| item.id != item_id);
                        self.filtered_items = self.search_filter.apply(&self.items);
                        self.alert_manager.update_from_inventory(&self.items);

                        // Log item deletion
                        if let Some(item_name) = deleted_item {
                            let audit_entry = AuditEntry::new(
                                session.user_id.clone(),
                                session.username.clone(),
                                AuditAction::ItemDeleted,
                                "item".to_string(),
                                Some(item_id.clone()),
                                format!("Deleted item: {}", item_name),
                            );
                            self.audit_log.add_entry(audit_entry);
                        }

                        return self.auto_save();
                    }
                }
            }

            // Notes Messages
            Message::CreateNote => {
                let new_note = Note::new("Untitled Note".to_string());
                self.selected_note_id = Some(new_note.id.clone());
                self.note_title_input = new_note.title.clone();
                self.editor_content = text_editor::Content::new();

                // Log note creation
                if let Some(session) = &self.session {
                    let audit_entry = AuditEntry::new(
                        session.user_id.clone(),
                        session.username.clone(),
                        AuditAction::NoteCreated,
                        "note".to_string(),
                        Some(new_note.id.clone()),
                        format!("Created note: {}", new_note.title),
                    );
                    self.audit_log.add_entry(audit_entry);
                }

                self.notes.push(new_note);
                return self.auto_save();
            }
            Message::SelectNote(note_id) => {
                if let Some(note) = self.notes.iter().find(|n| n.id == note_id) {
                    self.selected_note_id = Some(note.id.clone());
                    self.note_title_input = note.title.clone();
                    self.editor_content = text_editor::Content::with_text(&note.content);
                }
            }
            Message::UpdateNoteTitle(title) => {
                self.note_title_input = title.clone();
                if let Some(note_id) = &self.selected_note_id {
                    if let Some(note) = self.notes.iter_mut().find(|n| n.id == *note_id) {
                        note.update_title(title);
                        return self.auto_save();
                    }
                }
            }
            Message::UpdateNoteContent(action) => {
                self.editor_content.perform(action);
                if let Some(note_id) = &self.selected_note_id {
                    if let Some(note) = self.notes.iter_mut().find(|n| n.id == *note_id) {
                        note.update_content(self.editor_content.text());
                        return self.auto_save();
                    }
                }
            }
            Message::DeleteNote(note_id) => {
                self.delete_note_confirm = Some(note_id);
            }
            Message::ConfirmDeleteNote => {
                if let Some(note_id) = &self.delete_note_confirm {
                    let deleted_note = self
                        .notes
                        .iter()
                        .find(|n| n.id == *note_id)
                        .map(|n| n.title.clone());

                    self.notes.retain(|note| note.id != *note_id);

                    // Log note deletion
                    if let Some(session) = &self.session {
                        if let Some(note_title) = deleted_note {
                            let audit_entry = AuditEntry::new(
                                session.user_id.clone(),
                                session.username.clone(),
                                AuditAction::NoteDeleted,
                                "note".to_string(),
                                Some(note_id.clone()),
                                format!("Deleted note: {}", note_title),
                            );
                            self.audit_log.add_entry(audit_entry);
                        }
                    }

                    // If deleted note was selected, select another or clear
                    if self.selected_note_id.as_ref() == Some(note_id) {
                        if let Some(first_note) = self.notes.first() {
                            self.selected_note_id = Some(first_note.id.clone());
                            self.note_title_input = first_note.title.clone();
                            self.editor_content =
                                text_editor::Content::with_text(&first_note.content);
                        } else {
                            self.selected_note_id = None;
                            self.note_title_input.clear();
                            self.editor_content = text_editor::Content::new();
                        }
                    }

                    self.delete_note_confirm = None;
                    return self.auto_save();
                }
            }
            Message::CloseDeleteConfirm => {
                self.delete_note_confirm = None;
            }

            // Calculator Messages
            Message::ToggleCalculator => {
                self.calculator.toggle_visibility();
                // Set default position if not set
                if self.calculator.visible && self.calculator.position.is_none() {
                    self.calculator.set_position(450.0, 200.0);
                }
            }
            Message::CalculatorInput(digit) => {
                self.calculator.input_digit(digit);
            }
            Message::CalculatorOperation(op) => {
                self.calculator.set_operation(op);
            }
            Message::CalculatorEquals => {
                self.calculator.calculate_result();
            }
            Message::CalculatorClear => {
                self.calculator.clear();
            }
            Message::CalculatorDragStart => {
                self.calculator.dragging = true;
            }
            Message::CalculatorDragMove(x, y) => {
                if self.calculator.dragging {
                    // Adjust position to keep calculator centered on cursor
                    let calc_x = x - 150.0; // Half of calculator width (300 / 2)
                    let calc_y = y - 210.0; // Half of calculator height (420 / 2)
                    self.calculator.set_position(calc_x, calc_y);
                }
            }
            Message::CalculatorDragEnd => {
                self.calculator.end_drag();
                return self.auto_save();
            }

            // View Switching
            Message::SwitchView(view) => {
                self.current_view = view;
            }

            // App Actions
            Message::Save => {
                return self.auto_save();
            }
            Message::ShowAbout => {
                self.show_about = true;
            }
            Message::CloseAbout => {
                self.show_about = false;
            }
            Message::ConfirmClearAllData => {
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
                return self.auto_save();
            }
            Message::CancelClearAllData => {
                self.show_clear_confirm = false;
            }

            // Settings Messages
            Message::ToggleAutoSave => {
                self.settings.auto_save_enabled = !self.settings.auto_save_enabled;
                return self.auto_save();
            }
            Message::AutoSaveIntervalChanged(value) => {
                self.settings_interval_input = value.clone();
                if let Ok(interval) = value.parse::<u32>() {
                    if interval > 0 && interval <= 300 {
                        self.settings.auto_save_interval = interval;
                        return self.auto_save();
                    }
                }
            }
            Message::DefaultCategoryChanged(value) => {
                self.settings_category_input = value.clone();
                self.settings.default_category = value;
                return self.auto_save();
            }
            Message::ThemeChanged(theme) => {
                self.settings.theme = theme;
                return self.auto_save();
            }
            Message::ToggleLoadingScreen => {
                self.settings.show_loading_screen = !self.settings.show_loading_screen;
                return self.auto_save();
            }
            Message::ExportData => {
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
                return Task::batch(vec![self.auto_save(), task]);
            }
            Message::ImportData => {
                // For now, users can manually copy their exported file to the data location
                // A full implementation would require a file picker dialog
                return Task::perform(
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
                );
            }
            Message::ClearAllData => {
                self.show_clear_confirm = true;
            }
        }
        Task::none()
    }

    fn clear_item_inputs(&mut self) {
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

    fn auto_save(&self) -> Task<Message> {
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
        };
        Task::perform(
            async move {
                persistence::save_state(&state).await.ok();
            },
            |_| Message::Save,
        )
    }

    fn subscription(&self) -> Subscription<Message> {
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

    fn view(&self) -> Element<Message> {
        match self.state {
            AppState::Loading => views::loading::view(),
            AppState::Login => views::login::view(
                &self.username_input,
                &self.password_input,
                self.login_error.as_deref(),
            ),
            AppState::Loaded => {
                let main_content = self.view_loaded();

                // Stack overlays: main content -> item dialog -> about dialog
                let mut stack = vec![main_content];

                if let Some(ref mode) = self.item_dialog_mode {
                    stack.push(views::item_dialog::view(
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
                    ));
                }

                if self.show_about {
                    stack.push(views::about::view());
                }

                if self.show_clear_confirm {
                    stack.push(self.view_clear_confirm());
                }

                if stack.len() == 1 {
                    stack.into_iter().next().unwrap()
                } else {
                    iced::widget::stack(stack).into()
                }
            }
        }
    }

    fn view_loaded(&self) -> Element<Message> {
        let session = self
            .session
            .as_ref()
            .expect("Must be logged in to view app");

        let alert_count = self.alert_manager.get_unacknowledged_count();
        let alert_text = if alert_count > 0 {
            format!("🔔 Alerts ({})", alert_count)
        } else {
            "🔔 Alerts".to_string()
        };

        let mut header_buttons = vec![
            button("Inventory").on_press(Message::SwitchView(View::Inventory)),
            button("Notes").on_press(Message::SwitchView(View::Editor)),
            button(text(alert_text)).on_press(Message::SwitchView(View::Alerts)),
            button("Settings").on_press(Message::SwitchView(View::Settings)),
        ];

        // Only admins can access user management
        if session.role.can_manage_users() {
            header_buttons
                .push(button("Users").on_press(Message::SwitchView(View::UserManagement)));
        }

        // Only managers and admins can access audit log
        if session.role.can_view_audit() {
            header_buttons.push(button("Audit Log").on_press(Message::SwitchView(View::AuditLog)));
        }

        header_buttons.push(button("About").on_press(Message::ShowAbout));

        let user_info = text(format!(
            "Logged in as: {} ({})",
            session.username,
            format!("{:?}", session.role)
        ))
        .size(12)
        .style(|_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7)),
        });

        let logout_button = button("Logout").on_press(Message::Logout).padding(5).style(
            |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.6, 0.2, 0.2,
                    ))),
                    text_color: iced::Color::WHITE,
                    border: iced::Border {
                        radius: 3.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            },
        );

        let header_row: Vec<Element<Message>> =
            header_buttons.into_iter().map(|b| b.into()).collect();

        let header = column![
            row(header_row).spacing(10),
            row![user_info, logout_button,].spacing(10),
        ]
        .spacing(5)
        .padding(10);

        let ctrl_or_cmd = if cfg!(target_os = "macos") {
            "Cmd"
        } else {
            "Ctrl"
        };
        let shortcuts_text = format!(
            "{0}+1=Inventory | {0}+2=Notes | {0}+K=Calc | {0}+N=New Note | {0}+S=Save",
            ctrl_or_cmd
        );
        let shortcuts_hint =
            row![
                iced::widget::text(shortcuts_text)
                    .size(11)
                    .style(|_theme: &iced::Theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                        }
                    }),
            ]
            .padding(5);

        let content = match self.current_view {
            View::Inventory => views::inventory::view(
                &self.filtered_items,
                &self.items,
                &self.search_filter,
                self.show_search_panel,
            ),
            View::Editor => views::editor::view(
                &self.notes,
                self.selected_note_id.as_ref(),
                &self.editor_content,
                &self.note_title_input,
                self.delete_note_confirm.as_ref(),
            ),
            View::Settings => views::settings::view(
                &self.settings,
                &self.settings_interval_input,
                &self.settings_category_input,
            ),
            View::UserManagement => {
                let users: Vec<_> = self.auth_store.get_all_users();
                views::user_management::view(
                    &users,
                    session.role,
                    &self.new_username_input,
                    &self.new_password_input,
                    self.new_role_input,
                    self.user_operation_error.as_deref(),
                )
            }
            View::AuditLog => {
                let entries = self.audit_log.get_recent(100);
                views::audit_log::view(&entries, session.role)
            }
            View::Alerts => views::alerts::view(&self.alert_manager, session.role),
        };

        let main_view = column![header, shortcuts_hint, content]
            .spacing(5)
            .padding(10)
            .height(Length::Fill);

        if self.calculator.visible {
            let calculator = views::calculator::view(&self.calculator.display);

            let positioned_calc =
                container(calculator)
                    .width(300)
                    .padding(20)
                    .style(|_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(
                            0.15, 0.15, 0.15,
                        ))),
                        border: iced::Border {
                            color: if self.calculator.dragging {
                                iced::Color::from_rgb(0.3, 0.5, 0.7)
                            } else {
                                iced::Color::from_rgb(0.5, 0.5, 0.5)
                            },
                            width: 2.0,
                            radius: 10.0.into(),
                        },
                        ..Default::default()
                    });

            // Stack calculator on top of main view
            container(iced::widget::stack![
                main_view,
                container(positioned_calc)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
            ])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            main_view.into()
        }
    }

    fn delay(seconds: u64) {
        std::thread::sleep(std::time::Duration::from_secs(seconds));
    }

    fn view_clear_confirm(&self) -> Element<Message> {
        use iced::widget::{button, column, container, row, text};

        container(
            container(
                column![
                    text("Clear All Data?").size(24),
                    text("").size(10),
                    text("This will permanently delete:").size(14),
                    text("• All inventory items").size(13),
                    text("• All notes").size(13),
                    text("").size(10),
                    text("This action cannot be undone!")
                        .size(14)
                        .style(|_theme: &iced::Theme| {
                            text::Style {
                                color: Some(iced::Color::from_rgb(0.9, 0.3, 0.3)),
                            }
                        }),
                    text("").size(20),
                    row![
                        button("Yes, Delete Everything")
                            .on_press(Message::ConfirmClearAllData)
                            .padding(10)
                            .style(|_theme: &iced::Theme, _status: button::Status| {
                                button::Style {
                                    background: Some(iced::Background::Color(
                                        iced::Color::from_rgb(0.7, 0.2, 0.2),
                                    )),
                                    text_color: iced::Color::WHITE,
                                    border: iced::Border {
                                        radius: 5.0.into(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                            }),
                        button("Cancel")
                            .on_press(Message::CancelClearAllData)
                            .padding(10)
                            .style(|_theme: &iced::Theme, _status: button::Status| {
                                button::Style {
                                    background: Some(iced::Background::Color(
                                        iced::Color::from_rgb(0.3, 0.3, 0.3),
                                    )),
                                    text_color: iced::Color::WHITE,
                                    border: iced::Border {
                                        radius: 5.0.into(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                            }),
                    ]
                    .spacing(10),
                ]
                .spacing(5)
                .padding(30),
            )
            .width(400)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.15, 0.15, 0.15,
                ))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                    width: 2.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            }),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x(iced::Length::Fill)
        .center_y(iced::Length::Fill)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(
                0.0, 0.0, 0.0, 0.7,
            ))),
            ..Default::default()
        })
        .into()
    }
}
