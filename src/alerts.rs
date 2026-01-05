use crate::inventory::InventoryItem;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    OutOfStock,
    LowStock,
    CriticallyLow,
}

impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertType::OutOfStock => write!(f, "Out of Stock"),
            AlertType::LowStock => write!(f, "Low Stock"),
            AlertType::CriticallyLow => write!(f, "Critically Low"),
        }
    }
}

impl AlertType {
    pub fn color(&self) -> iced::Color {
        match self {
            AlertType::OutOfStock => iced::Color::from_rgb(0.9, 0.3, 0.3),
            AlertType::LowStock => iced::Color::from_rgb(0.9, 0.7, 0.3),
            AlertType::CriticallyLow => iced::Color::from_rgb(0.9, 0.5, 0.2),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            AlertType::OutOfStock => "🚫",
            AlertType::LowStock => "⚠️",
            AlertType::CriticallyLow => "❗",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAlert {
    pub id: String,
    pub item_id: String,
    pub item_name: String,
    pub item_sku: String,
    pub current_quantity: u32,
    pub alert_type: AlertType,
    pub threshold: u32,
    pub created_at: i64,
    pub acknowledged: bool,
}

impl StockAlert {
    pub fn new(
        item: &InventoryItem,
        alert_type: AlertType,
        threshold: u32,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_id: item.id.clone(),
            item_name: item.name.clone(),
            item_sku: item.sku.clone(),
            current_quantity: item.quantity,
            alert_type,
            threshold,
            created_at: Utc::now().timestamp(),
            acknowledged: false,
        }
    }

    pub fn formatted_timestamp(&self) -> String {
        chrono::DateTime::from_timestamp(self.created_at, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSettings {
    pub enabled: bool,
    pub low_stock_threshold: u32,
    pub critically_low_threshold: u32,
    pub show_notifications: bool,
}

impl Default for AlertSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            low_stock_threshold: 10,
            critically_low_threshold: 3,
            show_notifications: true,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlertManager {
    active_alerts: Vec<StockAlert>,
    alert_history: Vec<StockAlert>,
    settings: AlertSettings,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            active_alerts: Vec::new(),
            alert_history: Vec::new(),
            settings: AlertSettings::default(),
        }
    }

    pub fn settings(&self) -> &AlertSettings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut AlertSettings {
        &mut self.settings
    }

    pub fn update_from_inventory(&mut self, items: &[InventoryItem]) {
        if !self.settings.enabled {
            self.active_alerts.clear();
            return;
        }

        let mut new_alerts = Vec::new();

        for item in items {
            // Check if alert already exists
            let existing = self.active_alerts.iter().any(|a| a.item_id == item.id);

            if existing {
                // Update existing alert quantity
                if let Some(alert) = self.active_alerts.iter_mut().find(|a| a.item_id == item.id) {
                    alert.current_quantity = item.quantity;
                    
                    // Check if item is back in stock and remove alert
                    if item.quantity >= self.settings.low_stock_threshold {
                        alert.acknowledged = true;
                    }
                }
                continue;
            }

            // Create new alert if needed
            if item.quantity == 0 {
                new_alerts.push(StockAlert::new(item, AlertType::OutOfStock, 0));
            } else if item.quantity <= self.settings.critically_low_threshold {
                new_alerts.push(StockAlert::new(
                    item,
                    AlertType::CriticallyLow,
                    self.settings.critically_low_threshold,
                ));
            } else if item.quantity <= self.settings.low_stock_threshold {
                new_alerts.push(StockAlert::new(
                    item,
                    AlertType::LowStock,
                    self.settings.low_stock_threshold,
                ));
            }
        }

        self.active_alerts.extend(new_alerts);

        // Remove acknowledged alerts and move to history
        let acknowledged = self.active_alerts.drain_filter(|a| a.acknowledged);
        self.alert_history.extend(acknowledged);

        // Keep only last 100 historical alerts
        if self.alert_history.len() > 100 {
            self.alert_history.drain(0..self.alert_history.len() - 100);
        }
    }

    pub fn get_active_alerts(&self) -> &[StockAlert] {
        &self.active_alerts
    }

    pub fn get_unacknowledged_count(&self) -> usize {
        self.active_alerts.iter().filter(|a| !a.acknowledged).count()
    }

    pub fn acknowledge_alert(&mut self, alert_id: &str) {
        if let Some(alert) = self.active_alerts.iter_mut().find(|a| a.id == *alert_id) {
            alert.acknowledged = true;
        }
    }

    pub fn acknowledge_all(&mut self) {
        for alert in &mut self.active_alerts {
            alert.acknowledged = true;
        }
    }

    pub fn clear_acknowledged(&mut self) {
        let acknowledged = self.active_alerts.drain_filter(|a| a.acknowledged);
        self.alert_history.extend(acknowledged);
    }

    pub fn get_alert_history(&self) -> &[StockAlert] {
        &self.alert_history
    }

    pub fn get_critical_alerts(&self) -> Vec<&StockAlert> {
        self.active_alerts
            .iter()
            .filter(|a| {
                matches!(a.alert_type, AlertType::OutOfStock | AlertType::CriticallyLow)
                    && !a.acknowledged
            })
            .collect()
    }
}

// Note: drain_filter is nightly-only, so let's use a workaround
trait DrainFilter<T> {
    fn drain_filter<F>(&mut self, f: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool;
}

impl<T> DrainFilter<T> for Vec<T> {
    fn drain_filter<F>(&mut self, mut f: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut i = 0;
        let mut removed = Vec::new();
        
        while i < self.len() {
            if f(&self[i]) {
                removed.push(self.remove(i));
            } else {
                i += 1;
            }
        }
        
        removed
    }
}

