use std::fmt;

#[derive(Debug, Clone)]
pub enum ValidationError {
    FieldRequired(String),
    InvalidLength {
        field: String,
        min: usize,
        max: usize,
        actual: usize,
    },
    InvalidFormat {
        field: String,
        expected: String,
    },
    DuplicateValue {
        field: String,
        value: String,
    },
    InvalidRange {
        field: String,
        min: f64,
        max: f64,
        actual: f64,
    },
    InvalidInteger {
        field: String,
        value: String,
    },
    InvalidDecimal {
        field: String,
        value: String,
    },
    Custom(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::FieldRequired(field) => {
                write!(f, "{} is required", field)
            }
            ValidationError::InvalidLength {
                field,
                min,
                max,
                actual,
            } => {
                write!(
                    f,
                    "{} must be between {} and {} characters (got {})",
                    field, min, max, actual
                )
            }
            ValidationError::InvalidFormat { field, expected } => {
                write!(f, "{} has invalid format. Expected: {}", field, expected)
            }
            ValidationError::DuplicateValue { field, value } => {
                write!(f, "{} '{}' already exists", field, value)
            }
            ValidationError::InvalidRange {
                field,
                min,
                max,
                actual,
            } => {
                write!(
                    f,
                    "{} must be between {} and {} (got {})",
                    field, min, max, actual
                )
            }
            ValidationError::InvalidInteger { field, value } => {
                write!(f, "{} must be a valid integer (got '{}')", field, value)
            }
            ValidationError::InvalidDecimal { field, value } => {
                write!(
                    f,
                    "{} must be a valid decimal number (got '{}')",
                    field, value
                )
            }
            ValidationError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

pub type ValidationResult<T> = Result<T, ValidationError>;

// Validator functions
pub fn validate_required(field: &str, value: &str) -> ValidationResult<()> {
    if value.trim().is_empty() {
        Err(ValidationError::FieldRequired(field.to_string()))
    } else {
        Ok(())
    }
}

pub fn validate_length(field: &str, value: &str, min: usize, max: usize) -> ValidationResult<()> {
    let len = value.len();
    if len < min || len > max {
        Err(ValidationError::InvalidLength {
            field: field.to_string(),
            min,
            max,
            actual: len,
        })
    } else {
        Ok(())
    }
}

pub fn validate_sku_format(sku: &str) -> ValidationResult<()> {
    if sku.is_empty() {
        return Err(ValidationError::FieldRequired("SKU".to_string()));
    }

    // SKU should be alphanumeric with optional hyphens/underscores
    let valid = sku
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_');

    if !valid {
        Err(ValidationError::InvalidFormat {
            field: "SKU".to_string(),
            expected: "alphanumeric characters with optional hyphens or underscores".to_string(),
        })
    } else if sku.len() > 50 {
        Err(ValidationError::InvalidLength {
            field: "SKU".to_string(),
            min: 1,
            max: 50,
            actual: sku.len(),
        })
    } else {
        Ok(())
    }
}

pub fn validate_price(value_str: &str) -> ValidationResult<f64> {
    if value_str.trim().is_empty() {
        return Err(ValidationError::FieldRequired("Price".to_string()));
    }

    let value: f64 = value_str
        .parse()
        .map_err(|_| ValidationError::InvalidDecimal {
            field: "Price".to_string(),
            value: value_str.to_string(),
        })?;

    if value < 0.0 {
        return Err(ValidationError::InvalidRange {
            field: "Price".to_string(),
            min: 0.0,
            max: 1_000_000.0,
            actual: value,
        });
    }

    if value > 1_000_000.0 {
        return Err(ValidationError::InvalidRange {
            field: "Price".to_string(),
            min: 0.0,
            max: 1_000_000.0,
            actual: value,
        });
    }

    // Check for maximum 2 decimal places
    let decimal_str = format!("{:.10}", value);
    if let Some(dot_pos) = decimal_str.find('.') {
        let after_dot = &decimal_str[dot_pos + 1..];
        if after_dot.trim_end_matches('0').len() > 2 {
            return Err(ValidationError::Custom(
                "Price can have at most 2 decimal places".to_string(),
            ));
        }
    }

    Ok(value)
}

pub fn validate_quantity(value_str: &str) -> ValidationResult<u32> {
    if value_str.trim().is_empty() {
        return Err(ValidationError::FieldRequired("Quantity".to_string()));
    }

    let value: u32 = value_str
        .parse()
        .map_err(|_| ValidationError::InvalidInteger {
            field: "Quantity".to_string(),
            value: value_str.to_string(),
        })?;

    if value > 1_000_000 {
        return Err(ValidationError::InvalidRange {
            field: "Quantity".to_string(),
            min: 0.0,
            max: 1_000_000.0,
            actual: value as f64,
        });
    }

    Ok(value)
}

pub fn check_duplicate_sku(
    sku: &str,
    items: &[crate::inventory::InventoryItem],
    exclude_id: Option<&str>,
) -> ValidationResult<()> {
    let duplicate = items.iter().any(|item| {
        item.sku.eq_ignore_ascii_case(sku) && exclude_id.map_or(true, |id| item.id != id)
    });

    if duplicate {
        Err(ValidationError::DuplicateValue {
            field: "SKU".to_string(),
            value: sku.to_string(),
        })
    } else {
        Ok(())
    }
}

// Fuzzy matching for similar items
pub fn find_similar_items(name: &str, items: &[crate::inventory::InventoryItem]) -> Vec<String> {
    let name_lower = name.to_lowercase();
    items
        .iter()
        .filter(|item| {
            let item_name_lower = item.name.to_lowercase();
            // Simple similarity: check if names share significant words
            let name_words: Vec<&str> = name_lower.split_whitespace().collect();
            let item_words: Vec<&str> = item_name_lower.split_whitespace().collect();

            let common_words = name_words
                .iter()
                .filter(|w| w.len() > 3) // Only consider words longer than 3 chars
                .filter(|w| item_words.contains(w))
                .count();

            common_words > 0
                && (item_name_lower.contains(&name_lower)
                    || name_lower.contains(&item_name_lower)
                    || common_words >= 2)
        })
        .map(|item| format!("{} (SKU: {})", item.name, item.sku))
        .collect()
}
