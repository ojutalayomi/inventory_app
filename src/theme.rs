use iced::Color;
use crate::messages::AppTheme;

// ============================================================================
// DESIGN SYSTEM CONSTANTS
// ============================================================================

// Spacing Scale
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 12.0;
pub const SPACING_LG: f32 = 16.0;
pub const SPACING_XL: f32 = 24.0;
pub const SPACING_2XL: f32 = 32.0;
pub const SPACING_3XL: f32 = 48.0;

// Typography Scale
pub const TEXT_DISPLAY: f32 = 40.0;
pub const TEXT_H1: f32 = 32.0;
pub const TEXT_H2: f32 = 24.0;
pub const TEXT_H3: f32 = 20.0;
pub const TEXT_BODY_LARGE: f32 = 16.0;
pub const TEXT_BODY: f32 = 14.0;
pub const TEXT_CAPTION: f32 = 12.0;

// Border Radius
pub const RADIUS_SM: f32 = 6.0;
pub const RADIUS_MD: f32 = 8.0;
pub const RADIUS_LG: f32 = 12.0;
pub const RADIUS_XL: f32 = 16.0;
pub const RADIUS_FULL: f32 = 9999.0;

// ============================================================================
// DARK MODE COLORS
// ============================================================================

// Backgrounds
pub const DARK_BG: Color = Color::from_rgb(0.059, 0.090, 0.165); // #0F172A
pub const DARK_SURFACE: Color = Color::from_rgb(0.118, 0.161, 0.235); // #1E293B
pub const DARK_SURFACE_ELEVATED: Color = Color::from_rgb(0.149, 0.196, 0.278); // #263548

// Primary Colors
pub const DARK_PRIMARY: Color = Color::from_rgb(0.231, 0.510, 0.965); // #3B82F6
pub const DARK_PRIMARY_DARK: Color = Color::from_rgb(0.145, 0.361, 0.804); // #255CD1
pub const DARK_PRIMARY_GRADIENT_START: Color = Color::from_rgb(0.231, 0.510, 0.965); // #3B82F6
pub const DARK_PRIMARY_GRADIENT_END: Color = Color::from_rgb(0.545, 0.361, 0.965); // #8B5CF6

// Semantic Colors
pub const DARK_SUCCESS: Color = Color::from_rgb(0.063, 0.725, 0.506); // #10B981
pub const DARK_WARNING: Color = Color::from_rgb(0.961, 0.620, 0.043); // #F59E0B
pub const DARK_DANGER: Color = Color::from_rgb(0.937, 0.267, 0.267); // #EF4444
pub const DARK_ACCENT: Color = Color::from_rgb(0.024, 0.714, 0.831); // #06B6D4

// Text Colors
pub const DARK_TEXT_PRIMARY: Color = Color::from_rgb(1.0, 1.0, 1.0); // #FFFFFF
pub const DARK_TEXT_SECONDARY: Color = Color::from_rgb(0.796, 0.835, 0.882); // #CBD5E1
pub const DARK_TEXT_TERTIARY: Color = Color::from_rgb(0.580, 0.639, 0.722); // #94A3B8

// ============================================================================
// LIGHT MODE COLORS
// ============================================================================

// Backgrounds
pub const LIGHT_BG: Color = Color::from_rgb(0.973, 0.980, 0.988); // #F8FAFC
pub const LIGHT_SURFACE: Color = Color::from_rgb(1.0, 1.0, 1.0); // #FFFFFF
pub const LIGHT_SURFACE_ELEVATED: Color = Color::from_rgb(0.980, 0.988, 0.996); // #FAFCFE

// Primary Colors
pub const LIGHT_PRIMARY: Color = Color::from_rgb(0.145, 0.388, 0.922); // #2563EB
pub const LIGHT_PRIMARY_DARK: Color = Color::from_rgb(0.118, 0.306, 0.765); // #1E4EC3
pub const LIGHT_PRIMARY_GRADIENT_START: Color = Color::from_rgb(0.145, 0.388, 0.922); // #2563EB
pub const LIGHT_PRIMARY_GRADIENT_END: Color = Color::from_rgb(0.486, 0.227, 0.929); // #7C3AED

// Semantic Colors
pub const LIGHT_SUCCESS: Color = Color::from_rgb(0.024, 0.588, 0.412); // #059669
pub const LIGHT_WARNING: Color = Color::from_rgb(0.851, 0.467, 0.024); // #D97706
pub const LIGHT_DANGER: Color = Color::from_rgb(0.863, 0.149, 0.149); // #DC2626
pub const LIGHT_ACCENT: Color = Color::from_rgb(0.051, 0.580, 0.533); // #0D9488

// Text Colors
pub const LIGHT_TEXT_PRIMARY: Color = Color::from_rgb(0.059, 0.090, 0.165); // #0F172A
pub const LIGHT_TEXT_SECONDARY: Color = Color::from_rgb(0.392, 0.455, 0.545); // #64748B
pub const LIGHT_TEXT_TERTIARY: Color = Color::from_rgb(0.580, 0.639, 0.722); // #94A3B8

// ============================================================================
// THEME HELPER FUNCTIONS
// ============================================================================

pub fn bg_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_BG,
        AppTheme::Light => LIGHT_BG,
    }
}

pub fn surface_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_SURFACE,
        AppTheme::Light => LIGHT_SURFACE,
    }
}

pub fn surface_elevated_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_SURFACE_ELEVATED,
        AppTheme::Light => LIGHT_SURFACE_ELEVATED,
    }
}

pub fn primary_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_PRIMARY,
        AppTheme::Light => LIGHT_PRIMARY,
    }
}

pub fn primary_dark_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_PRIMARY_DARK,
        AppTheme::Light => LIGHT_PRIMARY_DARK,
    }
}

pub fn success_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_SUCCESS,
        AppTheme::Light => LIGHT_SUCCESS,
    }
}

pub fn warning_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_WARNING,
        AppTheme::Light => LIGHT_WARNING,
    }
}

pub fn danger_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_DANGER,
        AppTheme::Light => LIGHT_DANGER,
    }
}

pub fn accent_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_ACCENT,
        AppTheme::Light => LIGHT_ACCENT,
    }
}

pub fn text_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_TEXT_PRIMARY,
        AppTheme::Light => LIGHT_TEXT_PRIMARY,
    }
}

pub fn text_secondary_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_TEXT_SECONDARY,
        AppTheme::Light => LIGHT_TEXT_SECONDARY,
    }
}

pub fn text_tertiary_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => DARK_TEXT_TERTIARY,
        AppTheme::Light => LIGHT_TEXT_TERTIARY,
    }
}

pub fn border_color(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => Color::from_rgba(1.0, 1.0, 1.0, 0.1),
        AppTheme::Light => Color::from_rgba(0.0, 0.0, 0.0, 0.1),
    }
}

pub fn hover_overlay(theme: &AppTheme) -> Color {
    match theme {
        AppTheme::Dark => Color::from_rgba(1.0, 1.0, 1.0, 0.05),
        AppTheme::Light => Color::from_rgba(0.0, 0.0, 0.0, 0.03),
    }
}

// Gradient helper - returns start and end colors for primary gradient
pub fn primary_gradient(theme: &AppTheme) -> (Color, Color) {
    match theme {
        AppTheme::Dark => (DARK_PRIMARY_GRADIENT_START, DARK_PRIMARY_GRADIENT_END),
        AppTheme::Light => (LIGHT_PRIMARY_GRADIENT_START, LIGHT_PRIMARY_GRADIENT_END),
    }
}

// Stock status colors
pub fn stock_status_color(status: &str, theme: &AppTheme) -> Color {
    match status {
        "Out of Stock" => danger_color(theme),
        "Critically Low" => danger_color(theme),
        "Low Stock" => warning_color(theme),
        "In Stock" => success_color(theme),
        _ => text_secondary_color(theme),
    }
}

// Category color helpers (return different colors for different categories)
pub fn category_color(category: &str, theme: &AppTheme) -> Color {
    let hash = category.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
    let hue = (hash % 360) as f32;
    
    match theme {
        AppTheme::Dark => Color::from_rgb(
            0.5 + (hue / 360.0) * 0.5,
            0.5 + ((hue + 120.0) % 360.0 / 360.0) * 0.5,
            0.5 + ((hue + 240.0) % 360.0 / 360.0) * 0.5,
        ),
        AppTheme::Light => Color::from_rgb(
            0.3 + (hue / 360.0) * 0.4,
            0.3 + ((hue + 120.0) % 360.0 / 360.0) * 0.4,
            0.3 + ((hue + 240.0) % 360.0 / 360.0) * 0.4,
        ),
    }
}

