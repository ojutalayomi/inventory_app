use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: String,
    pub name: String,
    pub sku: String,
    pub category: String,
    pub supplier: String,
    pub description: String,
    pub quantity: u32,
    pub price: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl InventoryItem {
    pub fn new(
        name: String,
        sku: String,
        category: String,
        supplier: String,
        description: String,
        quantity: u32,
        price: f64,
    ) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            sku,
            category,
            supplier,
            description,
            quantity,
            price,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now().timestamp();
    }

    pub fn total_value(&self) -> f64 {
        self.quantity as f64 * self.price
    }
}
