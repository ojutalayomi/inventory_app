use crate::inventory::InventoryItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortField {
    Name,
    Sku,
    Category,
    Supplier,
    Quantity,
    Price,
    CreatedAt,
    UpdatedAt,
}

impl std::fmt::Display for SortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortField::Name => write!(f, "Name"),
            SortField::Sku => write!(f, "SKU"),
            SortField::Category => write!(f, "Category"),
            SortField::Supplier => write!(f, "Supplier"),
            SortField::Quantity => write!(f, "Quantity"),
            SortField::Price => write!(f, "Price"),
            SortField::CreatedAt => write!(f, "Created Date"),
            SortField::UpdatedAt => write!(f, "Updated Date"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Ascending => write!(f, "↑ Asc"),
            SortDirection::Descending => write!(f, "↓ Desc"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchFilter {
    pub query: String,
    pub category_filter: Option<String>,
    pub supplier_filter: Option<String>,
    pub min_quantity: Option<u32>,
    pub max_quantity: Option<u32>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub sort_field: Option<SortField>,
    pub sort_direction: SortDirection,
}

impl SearchFilter {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            category_filter: None,
            supplier_filter: None,
            min_quantity: None,
            max_quantity: None,
            min_price: None,
            max_price: None,
            sort_field: Some(SortField::Name),
            sort_direction: SortDirection::Ascending,
        }
    }

    pub fn is_active(&self) -> bool {
        !self.query.is_empty()
            || self.category_filter.is_some()
            || self.supplier_filter.is_some()
            || self.min_quantity.is_some()
            || self.max_quantity.is_some()
            || self.min_price.is_some()
            || self.max_price.is_some()
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.category_filter = None;
        self.supplier_filter = None;
        self.min_quantity = None;
        self.max_quantity = None;
        self.min_price = None;
        self.max_price = None;
    }

    pub fn matches(&self, item: &InventoryItem) -> bool {
        // Text search (searches in name, SKU, category, supplier, description)
        if !self.query.is_empty() {
            let query_lower = self.query.to_lowercase();
            let matches = item.name.to_lowercase().contains(&query_lower)
                || item.sku.to_lowercase().contains(&query_lower)
                || item.category.to_lowercase().contains(&query_lower)
                || item.supplier.to_lowercase().contains(&query_lower)
                || item.description.to_lowercase().contains(&query_lower);
            
            if !matches {
                return false;
            }
        }

        // Category filter
        if let Some(ref category) = self.category_filter {
            if !category.is_empty() && !item.category.eq_ignore_ascii_case(category) {
                return false;
            }
        }

        // Supplier filter
        if let Some(ref supplier) = self.supplier_filter {
            if !supplier.is_empty() && !item.supplier.eq_ignore_ascii_case(supplier) {
                return false;
            }
        }

        // Quantity range
        if let Some(min) = self.min_quantity {
            if item.quantity < min {
                return false;
            }
        }
        if let Some(max) = self.max_quantity {
            if item.quantity > max {
                return false;
            }
        }

        // Price range
        if let Some(min) = self.min_price {
            if item.price < min {
                return false;
            }
        }
        if let Some(max) = self.max_price {
            if item.price > max {
                return false;
            }
        }

        true
    }

    pub fn apply(&self, items: &[InventoryItem]) -> Vec<InventoryItem> {
        let mut filtered: Vec<InventoryItem> = items
            .iter()
            .filter(|item| self.matches(item))
            .cloned()
            .collect();

        // Sort
        if let Some(ref field) = self.sort_field {
            filtered.sort_by(|a, b| {
                let cmp = match field {
                    SortField::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    SortField::Sku => a.sku.to_lowercase().cmp(&b.sku.to_lowercase()),
                    SortField::Category => a.category.to_lowercase().cmp(&b.category.to_lowercase()),
                    SortField::Supplier => a.supplier.to_lowercase().cmp(&b.supplier.to_lowercase()),
                    SortField::Quantity => a.quantity.cmp(&b.quantity),
                    SortField::Price => a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal),
                    SortField::CreatedAt => a.created_at.cmp(&b.created_at),
                    SortField::UpdatedAt => a.updated_at.cmp(&b.updated_at),
                };

                match self.sort_direction {
                    SortDirection::Ascending => cmp,
                    SortDirection::Descending => cmp.reverse(),
                }
            });
        }

        filtered
    }

    pub fn get_unique_categories(items: &[InventoryItem]) -> Vec<String> {
        let mut categories: Vec<String> = items
            .iter()
            .map(|item| item.category.clone())
            .filter(|c| !c.is_empty())
            .collect();
        categories.sort();
        categories.dedup();
        categories
    }

    pub fn get_unique_suppliers(items: &[InventoryItem]) -> Vec<String> {
        let mut suppliers: Vec<String> = items
            .iter()
            .map(|item| item.supplier.clone())
            .filter(|s| !s.is_empty())
            .collect();
        suppliers.sort();
        suppliers.dedup();
        suppliers
    }
}

