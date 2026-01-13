use iced::widget::{container, svg, text};
use iced::{Color, Element, Length};
use std::path::PathBuf;

use crate::messages::{AppTheme, Message};
use crate::theme;

/// Icon size presets
#[derive(Debug, Clone, Copy)]
pub enum IconSize {
    Small,   // 16px
    Medium,  // 20px
    Large,   // 24px
    XLarge,  // 32px
}

impl IconSize {
    pub fn pixels(&self) -> f32 {
        match self {
            IconSize::Small => 16.0,
            IconSize::Medium => 20.0,
            IconSize::Large => 24.0,
            IconSize::XLarge => 32.0,
        }
    }
}

/// All available icons in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    // Navigation
    Inventory,
    Notes,
    Alerts,
    Settings,
    Users,
    AuditLog,
    About,
    
    // Actions
    Add,
    Edit,
    Delete,
    Search,
    Save,
    Check,
    Close,
    Filter,
    Logout,
    
    // Status
    CheckCircle,
    AlertTriangle,
    AlertCircle,
    XCircle,
    Info,
    
    // Misc
    User,
    Calculator,
    Chart,
    Dollar,
    Box,
    Lock,
    Lightbulb,
}

impl Icon {
    /// Get the file path for this icon
    fn path(&self) -> PathBuf {
        let base = PathBuf::from("assets/icons");
        match self {
            // Navigation icons (actually in actions/ folder)
            Icon::Inventory => base.join("actions/inventory.svg"),
            Icon::Notes => base.join("actions/notes.svg"),
            Icon::Alerts => base.join("actions/alerts.svg"),
            Icon::Settings => base.join("actions/settings.svg"),
            Icon::Users => base.join("actions/users.svg"),
            Icon::AuditLog => base.join("actions/audit-log.svg"),
            Icon::About => base.join("status/info.svg"), // Reuse info icon
            
            // Action icons (actually in nav/ folder with different names)
            Icon::Add => base.join("nav/plus.svg"),
            Icon::Edit => base.join("nav/pencil.svg"),
            Icon::Delete => base.join("nav/trash.svg"),
            Icon::Search => base.join("nav/magnify.svg"),
            Icon::Save => base.join("nav/floppy.svg"),
            Icon::Check => base.join("nav/checkmark.svg"),
            Icon::Close => base.join("nav/close.svg"),
            Icon::Filter => base.join("nav/magnify.svg"), // Reuse search icon
            Icon::Logout => base.join("nav/close.svg"), // Reuse close icon
            
            // Status icons
            Icon::CheckCircle => base.join("status/check-circle.svg"),
            Icon::AlertTriangle => base.join("status/alert-triangle.svg"),
            Icon::AlertCircle => base.join("status/alert-circle.svg"),
            Icon::XCircle => base.join("status/x-circle.svg"),
            Icon::Info => base.join("status/info.svg"),
            
            // Misc icons
            Icon::User => base.join("misc/user.svg"),
            Icon::Calculator => base.join("misc/calculator.svg"),
            Icon::Chart => base.join("misc/chart.svg"),
            Icon::Dollar => base.join("misc/dollar.svg"),
            Icon::Box => base.join("misc/box.svg"),
            Icon::Lock => base.join("misc/user.svg"), // Reuse user icon as placeholder
            Icon::Lightbulb => base.join("status/info.svg"), // Reuse info icon
        }
    }
    
    /// Get text fallback for when SVG doesn't exist or render
    fn text_fallback(&self) -> &'static str {
        match self {
            // Navigation - Use simple text labels
            Icon::Inventory => "INV",
            Icon::Notes => "NOTE",
            Icon::Alerts => "ALRT",
            Icon::Settings => "SETT",
            Icon::Users => "USER",
            Icon::AuditLog => "AUDT",
            Icon::About => "INFO",
            
            // Actions - Use symbols
            Icon::Add => "+",
            Icon::Edit => "ED",
            Icon::Delete => "DEL",
            Icon::Search => "?",
            Icon::Save => "SAV",
            Icon::Check => "✓",
            Icon::Close => "X",
            Icon::Filter => "FIL",
            Icon::Logout => "OUT",
            
            // Status - Use symbols
            Icon::CheckCircle => "✓",
            Icon::AlertTriangle => "!",
            Icon::AlertCircle => "!",
            Icon::XCircle => "X",
            Icon::Info => "i",
            
            // Misc - Use abbreviations
            Icon::User => "USR",
            Icon::Calculator => "=",
            Icon::Chart => "CHT",
            Icon::Dollar => "$",
            Icon::Box => "BOX",
            Icon::Lock => "LCK",
            Icon::Lightbulb => "*",
        }
    }
    
    /// Check if SVG file exists
    fn svg_exists(&self) -> bool {
        let path = self.path();
        let exists = path.exists();
        
        // Debug: Print if SVG doesn't exist
        if !exists {
            eprintln!("SVG not found: {:?}", path);
        }
        
        exists
    }
    
    /// Create a themed icon widget
    pub fn view<'a>(
        &self,
        size: IconSize,
        app_theme: &'a AppTheme,
    ) -> Element<'a, Message> {
        self.view_with_color(size, None, app_theme)
    }
    
    /// Create a themed icon widget with custom color
    pub fn view_with_color<'a>(
        &self,
        size: IconSize,
        custom_color: Option<Color>,
        app_theme: &'a AppTheme,
    ) -> Element<'a, Message> {
        let icon_size = size.pixels();
        
        // Try to load SVG, fallback to text
        if self.svg_exists() {
            let handle = svg::Handle::from_path(self.path());
            
            // Only apply color if custom color is specified, otherwise let SVG use its own colors
            let svg_widget = if let Some(color) = custom_color {
                svg(handle)
                    .width(icon_size)
                    .height(icon_size)
                    .style(move |_theme: &iced::Theme, _status: svg::Status| svg::Style {
                        color: Some(color),
                    })
            } else {
                // No color override - let SVG use its built-in colors
                svg(handle)
                    .width(icon_size)
                    .height(icon_size)
            };
            
            container(svg_widget)
                .width(icon_size)
                .height(icon_size)
                .into()
        } else {
            // Fallback to text label
            eprintln!("Using text fallback for icon: {:?}", self);
            let fallback_text = self.text_fallback();
            let text_size = (icon_size * 0.45).max(10.0); // Smaller for text labels
            let text_color = custom_color.unwrap_or_else(|| theme::text_color(app_theme));
            
            container(
                text(fallback_text)
                    .size(text_size)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(text_color),
                    })
            )
            .width(icon_size)
            .height(icon_size)
            .center_x(icon_size)
            .center_y(icon_size)
            .into()
        }
    }
    
    /// Get icon with text label
    pub fn view_with_label<'a>(
        &self,
        size: IconSize,
        label: &'a str,
        app_theme: &'a AppTheme,
    ) -> Element<'a, Message> {
        use iced::widget::{row};
        
        row![
            self.view(size, app_theme),
            text(label)
                .size(theme::TEXT_BODY)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme::text_color(app_theme)),
                })
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center)
        .into()
    }
}

/// Icon for illustrations (larger graphics)
pub enum Illustration {
    EmptyInventory,
    NoResults,
    NoAlerts,
    NoNotes,
}

impl Illustration {
    fn path(&self) -> PathBuf {
        let base = PathBuf::from("assets/illustrations");
        match self {
            Illustration::EmptyInventory => base.join("empty-inventory.svg"),
            Illustration::NoResults => base.join("no-results.svg"),
            Illustration::NoAlerts => base.join("no-alerts.svg"),
            Illustration::NoNotes => base.join("no-notes.svg"),
        }
    }
    
    fn fallback_emoji(&self) -> &'static str {
        match self {
            Illustration::EmptyInventory => "📦",
            Illustration::NoResults => "🔍",
            Illustration::NoAlerts => "🔔",
            Illustration::NoNotes => "📝",
        }
    }
    
    fn fallback_text(&self) -> &'static str {
        match self {
            Illustration::EmptyInventory => "No items yet",
            Illustration::NoResults => "No results found",
            Illustration::NoAlerts => "No alerts",
            Illustration::NoNotes => "No notes",
        }
    }
    
    fn svg_exists(&self) -> bool {
        self.path().exists()
    }
    
    pub fn view<'a>(
        &self,
        max_size: f32,
        app_theme: &'a AppTheme,
    ) -> Element<'a, Message> {
        use iced::widget::column;
        
        if self.svg_exists() {
            let handle = svg::Handle::from_path(self.path());
            
            container(
                svg(handle)
                    .width(max_size)
                    .height(max_size)
            )
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
        } else {
            // Fallback to emoji + text
            container(
                column![
                    text(self.fallback_emoji()).size(64),
                    text(self.fallback_text())
                        .size(theme::TEXT_H2)
                        .style(move |_theme: &iced::Theme| text::Style {
                            color: Some(theme::text_secondary_color(app_theme)),
                        }),
                ]
                .spacing(theme::SPACING_LG)
                .align_x(iced::Alignment::Center)
            )
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
        }
    }
}

/// Logo variants
pub enum Logo {
    AppIcon,
    Full,
    Small,
}

impl Logo {
    fn path(&self) -> PathBuf {
        let base = PathBuf::from("assets/logos");
        match self {
            Logo::AppIcon => base.join("app-icon.svg"),
            Logo::Full => base.join("logo.svg"),
            Logo::Small => base.join("logo-small.svg"),
        }
    }
    
    fn svg_exists(&self) -> bool {
        self.path().exists()
    }
    
    pub fn view<'a>(
        &self,
        size: f32,
        app_theme: &'a AppTheme,
    ) -> Element<'a, Message> {
        if self.svg_exists() {
            let handle = svg::Handle::from_path(self.path());
            
            svg(handle)
                .width(size)
                .height(size)
                .into()
        } else {
            // Fallback
            text("📦 Inventory Manager")
                .size(size * 0.3)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme::primary_color(app_theme)),
                })
                .into()
        }
    }
}

