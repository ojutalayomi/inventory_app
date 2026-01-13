use crate::InventoryApp;
use crate::search::SortField;

impl InventoryApp {
    pub fn handle_toggle_search_panel(&mut self) {
        self.show_search_panel = !self.show_search_panel;
    }

    pub fn handle_search_query_changed(&mut self, query: String) {
        self.search_filter.query = query;
        self.filtered_items = self.search_filter.apply(&self.items);
    }

    pub fn handle_category_filter_changed(&mut self, category: String) {
        self.search_filter.category_filter = if category.is_empty() {
            None
        } else {
            Some(category)
        };
        self.filtered_items = self.search_filter.apply(&self.items);
    }

    pub fn handle_supplier_filter_changed(&mut self, supplier: String) {
        self.search_filter.supplier_filter = if supplier.is_empty() {
            None
        } else {
            Some(supplier)
        };
        self.filtered_items = self.search_filter.apply(&self.items);
    }

    pub fn handle_min_quantity_changed(&mut self, value: String) {
        self.search_filter.min_quantity = value.parse().ok();
        self.filtered_items = self.search_filter.apply(&self.items);
    }

    pub fn handle_max_quantity_changed(&mut self, value: String) {
        self.search_filter.max_quantity = value.parse().ok();
        self.filtered_items = self.search_filter.apply(&self.items);
    }

    pub fn handle_min_price_changed(&mut self, value: String) {
        self.search_filter.min_price = value.parse().ok();
        self.filtered_items = self.search_filter.apply(&self.items);
    }

    pub fn handle_max_price_changed(&mut self, value: String) {
        self.search_filter.max_price = value.parse().ok();
        self.filtered_items = self.search_filter.apply(&self.items);
    }

    pub fn handle_sort_field_changed(&mut self, field: SortField) {
        self.search_filter.sort_field = Some(field);
        self.filtered_items = self.search_filter.apply(&self.items);
    }

    pub fn handle_sort_direction_toggled(&mut self) {
        self.search_filter.sort_direction = match self.search_filter.sort_direction {
            crate::search::SortDirection::Ascending => crate::search::SortDirection::Descending,
            crate::search::SortDirection::Descending => crate::search::SortDirection::Ascending,
        };
        self.filtered_items = self.search_filter.apply(&self.items);
    }

    pub fn handle_clear_filters(&mut self) {
        self.search_filter.clear();
        self.filtered_items = self.search_filter.apply(&self.items);
    }
}

