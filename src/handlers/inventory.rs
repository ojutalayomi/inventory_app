use iced::Task;
use crate::{InventoryApp, Message};
use crate::messages::ItemDialogMode;
use crate::inventory::InventoryItem;
use crate::audit::{AuditAction, AuditEntry};

impl InventoryApp {
    pub fn handle_open_add_dialog(&mut self) {
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

    pub fn handle_open_edit_dialog(&mut self, item_id: String) {
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

    pub fn handle_close_item_dialog(&mut self) {
        self.item_dialog_mode = None;
        self.clear_item_inputs();
    }

    pub fn handle_name_changed(&mut self, value: String) {
        self.name_input = value.clone();
        self.item_validation_error = None;
        
        // Check for similar items as the user types
        if value.len() > 3 {
            self.similar_items_warning = crate::errors::find_similar_items(&value, &self.items);
        } else {
            self.similar_items_warning.clear();
        }
    }

    pub fn handle_sku_changed(&mut self, value: String) {
        self.sku_input = value;
        self.item_validation_error = None;
    }

    pub fn handle_category_changed(&mut self, value: String) {
        self.category_input = value;
        self.item_validation_error = None;
    }

    pub fn handle_supplier_changed(&mut self, value: String) {
        self.supplier_input = value;
        self.item_validation_error = None;
    }

    pub fn handle_description_changed(&mut self, value: String) {
        self.description_input = value;
        self.item_validation_error = None;
    }

    pub fn handle_quantity_changed(&mut self, value: String) {
        self.quantity_input = value;
        self.item_validation_error = None;
    }

    pub fn handle_price_changed(&mut self, value: String) {
        self.price_input = value;
        self.item_validation_error = None;
    }

    pub fn handle_submit_item(&mut self) -> Task<Message> {
        use crate::errors::*;
        
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
        self.auto_save()
    }

    pub fn handle_delete_item(&mut self, item_id: String) -> Task<Message> {
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
        Task::none()
    }
}

