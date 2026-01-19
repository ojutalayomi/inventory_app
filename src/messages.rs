use iced::widget::text_editor;

#[derive(Debug, Clone)]
pub enum Message {
    // Loading messages
    Loaded(Result<SavedState, LoadError>),

    // Inventory messages
    OpenAddDialog,
    OpenEditDialog(String), // Pass item ID instead of index
    CloseItemDialog,
    NameChanged(String),
    SkuChanged(String),
    CategoryChanged(String),
    SupplierChanged(String),
    DescriptionChanged(String),
    QuantityChanged(String),
    PriceChanged(String),
    SubmitItem,
    DeleteItem(String), // Changed to use ID
    ExportInventoryCsv,
    InventoryViewModeChanged(InventoryViewMode),

    // Editor/Notes messages
    CreateNote,
    SelectNote(String),
    UpdateNoteTitle(String),
    UpdateNoteContent(text_editor::Action),
    DeleteNote(String),
    ConfirmDeleteNote,
    CloseDeleteConfirm,
    ExportNote(NoteExportFormat),

    // Calculator messages
    ToggleCalculator,
    CalculatorInput(String),
    CalculatorOperation(CalculatorOp),
    CalculatorEquals,
    CalculatorClear,
    CalculatorDragStart,
    CalculatorDragMove(f32, f32),
    CalculatorDragEnd,

    // View switching
    SwitchView(View),

    // Authentication messages
    UsernameChanged(String),
    PasswordChanged(String),
    AttemptLogin,
    LoginSuccess,
    LoginFailed(String),
    Logout,

    // User management messages
    NewUsernameChanged(String),
    NewPasswordChanged(String),
    NewRoleChanged(crate::user::UserRole),
    CreateUser,
    EditUser(String),
    DeleteUser(String),
    UserOperationResult(Result<(), String>),

    // Audit log messages
    ExportAuditLog,

    // Search and filter messages
    ToggleSearchPanel,
    SearchQueryChanged(String),
    CategoryFilterChanged(String),
    SupplierFilterChanged(String),
    MinQuantityChanged(String),
    MaxQuantityChanged(String),
    MinPriceChanged(String),
    MaxPriceChanged(String),
    SortFieldChanged(crate::search::SortField),
    SortDirectionToggled,
    ClearFilters,

    // Alert messages
    ToggleAlertsPanel,
    AcknowledgeAlert(String),
    AcknowledgeAllAlerts,
    ClearAcknowledgedAlerts,
    UpdateAlertSettings,
    AlertLowStockThresholdChanged(String),
    AlertCriticalThresholdChanged(String),
    ToggleAlertsEnabled,
    ToggleAlertNotifications,

    // Settings messages
    ToggleAutoSave,
    AutoSaveIntervalChanged(String),
    DefaultCategoryChanged(String),
    CurrencyChanged(String),
    ThemeChanged(AppTheme),
    ToggleLoadingScreen,
    LayoutStyleChanged(LayoutStyle),
    ToggleDeviceNotifications,
    ToggleUpdateNotifications,
    NotificationThrottleChanged(String),
    ToggleSidebar,
    ExportData,
    ImportData,
    OpenImportFilePicker,
    ImportFileSelected(Option<std::path::PathBuf>),
    ClearAllData,
    ConfirmClearAllData,
    CancelClearAllData,

    // Update messages
    CheckForUpdates,
    UpdateCheckComplete(Result<Option<crate::update_checker::UpdateInfo>, String>),
    DownloadUpdate(String), // download_url
    InstallUpdate(std::path::PathBuf),
    UpdateDownloadFailed(String),
    CloseUpdateNotification,

    // App actions
    Save,
    ShowAbout,
    CloseAbout,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppTheme {
    Dark,
    Light,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutStyle {
    Header,
    Sidebar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryViewMode {
    Cards,
    Table,
}

impl Default for InventoryViewMode {
    fn default() -> Self {
        InventoryViewMode::Cards
    }
}

impl serde::Serialize for InventoryViewMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            InventoryViewMode::Cards => "cards",
            InventoryViewMode::Table => "table",
        })
    }
}

impl<'de> serde::Deserialize<'de> for InventoryViewMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "table" => InventoryViewMode::Table,
            _ => InventoryViewMode::Cards,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteExportFormat {
    Txt,
    Markdown,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        LayoutStyle::Header
    }
}

impl serde::Serialize for LayoutStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            LayoutStyle::Header => "header",
            LayoutStyle::Sidebar => "sidebar",
        })
    }
}

impl<'de> serde::Deserialize<'de> for LayoutStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "sidebar" => LayoutStyle::Sidebar,
            _ => LayoutStyle::Header,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ItemDialogMode {
    Add,
    Edit(String), // Store item ID being edited
}

#[derive(Debug, Clone)]
pub enum LoadError {
    FileNotFound,
    FormatError,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SavedState {
    pub items: Vec<crate::inventory::InventoryItem>,
    pub notes: Vec<crate::note::Note>,
    pub calculator_position: Option<(f32, f32)>,
    pub settings: AppSettings,
    #[serde(default)]
    pub auth_store: crate::auth::AuthStore,
    #[serde(default)]
    pub audit_log: crate::audit::AuditLog,
    #[serde(default)]
    pub alert_manager: crate::alerts::AlertManager,
    #[serde(default)]
    pub sidebar_collapsed: bool,
    #[serde(default)]
    pub show_alerts_panel: bool,
    #[serde(default)]
    pub show_search_panel: bool,
    #[serde(default)]
    pub current_view: View,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    pub auto_save_enabled: bool,
    pub auto_save_interval: u32, // seconds
    pub default_category: String,
    #[serde(default)]
    pub preferred_currency: String,
    pub theme: AppTheme,
    pub show_loading_screen: bool,
    #[serde(default)]
    pub layout_style: LayoutStyle,
    #[serde(default)]
    pub inventory_view_mode: InventoryViewMode,
    #[serde(default)]
    pub device_notifications_enabled: bool,
    #[serde(default)]
    pub update_notifications_enabled: bool,
    #[serde(default)]
    pub notification_throttle_seconds: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_save_enabled: true,
            auto_save_interval: 5,
            default_category: String::from("General"),
            preferred_currency: String::from("USD"),
            theme: AppTheme::Dark,
            show_loading_screen: true,
            layout_style: LayoutStyle::default(),
            inventory_view_mode: InventoryViewMode::default(),
            device_notifications_enabled: true,
            update_notifications_enabled: true,
            notification_throttle_seconds: 30,
        }
    }
}

impl serde::Serialize for AppTheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            AppTheme::Dark => "dark",
            AppTheme::Light => "light",
        })
    }
}

impl<'de> serde::Deserialize<'de> for AppTheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "light" => AppTheme::Light,
            _ => AppTheme::Dark,
        })
    }
}

#[derive(Debug, Clone)]
pub enum CalculatorOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Inventory,
    Editor,
    Settings,
    UserManagement,
    AuditLog,
    Alerts,
}

impl Default for View {
    fn default() -> Self {
        View::Inventory
    }
}

impl serde::Serialize for View {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            View::Inventory => "inventory",
            View::Editor => "editor",
            View::Settings => "settings",
            View::UserManagement => "user_management",
            View::AuditLog => "audit_log",
            View::Alerts => "alerts",
        })
    }
}

impl<'de> serde::Deserialize<'de> for View {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "editor" => View::Editor,
            "settings" => View::Settings,
            "user_management" => View::UserManagement,
            "audit_log" => View::AuditLog,
            "alerts" => View::Alerts,
            _ => View::Inventory,
        })
    }
}
