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

    // Editor/Notes messages
    CreateNote,
    SelectNote(String),
    UpdateNoteTitle(String),
    UpdateNoteContent(text_editor::Action),
    DeleteNote(String),
    ConfirmDeleteNote,
    CloseDeleteConfirm,

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
    ThemeChanged(AppTheme),
    ToggleLoadingScreen,
    LayoutStyleChanged(LayoutStyle),
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
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    pub auto_save_enabled: bool,
    pub auto_save_interval: u32, // seconds
    pub default_category: String,
    pub theme: AppTheme,
    pub show_loading_screen: bool,
    #[serde(default)]
    pub layout_style: LayoutStyle,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_save_enabled: true,
            auto_save_interval: 5,
            default_category: String::from("General"),
            theme: AppTheme::Dark,
            show_loading_screen: true,
            layout_style: LayoutStyle::default(),
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
