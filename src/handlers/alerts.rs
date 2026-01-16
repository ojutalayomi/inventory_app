use iced::Task;
use crate::{InventoryApp, Message};

impl InventoryApp {
    pub fn handle_toggle_alerts_panel(&mut self) -> Task<Message> {
        self.show_alerts_panel = !self.show_alerts_panel;
        self.auto_save()
    }

    pub fn handle_acknowledge_alert(&mut self, alert_id: String) -> Task<Message> {
        self.alert_manager.acknowledge_alert(&alert_id);
        self.auto_save()
    }

    pub fn handle_acknowledge_all_alerts(&mut self) -> Task<Message> {
        self.alert_manager.acknowledge_all();
        self.auto_save()
    }

    pub fn handle_clear_acknowledged_alerts(&mut self) -> Task<Message> {
        self.alert_manager.clear_acknowledged();
        self.auto_save()
    }

    pub fn handle_alert_low_stock_threshold_changed(&mut self, value: String) -> Task<Message> {
        if let Ok(threshold) = value.parse::<u32>() {
            self.alert_manager.settings_mut().low_stock_threshold = threshold;
            self.update_alerts_from_inventory();
            return self.auto_save();
        }
        Task::none()
    }

    pub fn handle_alert_critical_threshold_changed(&mut self, value: String) -> Task<Message> {
        if let Ok(threshold) = value.parse::<u32>() {
            self.alert_manager.settings_mut().critically_low_threshold = threshold;
            self.update_alerts_from_inventory();
            return self.auto_save();
        }
        Task::none()
    }

    pub fn handle_toggle_alerts_enabled(&mut self) -> Task<Message> {
        self.alert_manager.settings_mut().enabled = !self.alert_manager.settings().enabled;
        self.update_alerts_from_inventory();
        self.auto_save()
    }

    pub fn handle_toggle_alert_notifications(&mut self) -> Task<Message> {
        self.alert_manager.settings_mut().show_notifications = !self.alert_manager.settings().show_notifications;
        self.auto_save()
    }

    pub fn handle_update_alert_settings(&mut self) -> Task<Message> {
        self.update_alerts_from_inventory();
        self.auto_save()
    }
}

