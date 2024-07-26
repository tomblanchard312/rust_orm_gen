use std::collections::HashMap;

pub trait Validate {
    fn validate(&self) -> impl std::future::Future<Output = ValidationResult> + Send;
}
pub trait ValidateSchema {
    fn validate_schema(db_url: &str) -> Result<(), Box<dyn std::error::Error>>;
}
pub struct ValidationResult {
    pub errors: HashMap<String, Vec<String>>,
}

impl ValidationResult {
    pub fn new() -> Self {
        ValidationResult {
            errors: HashMap::new(),
        }
    }

    pub fn add_error(&mut self, field: &str, message: &str) {
        self.errors
            .entry(field.to_string())
            .or_insert_with(Vec::new)
            .push(message.to_string());
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}