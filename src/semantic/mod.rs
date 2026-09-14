// Semantic engine implementation
pub struct SemanticEngine {}

use std::collections::HashMap;

pub struct SchemaRegistry {
    tables: HashMap<String, Vec<String>>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn load_mock_schema(&mut self) {
        self.tables.insert(
            "users".to_string(),
            vec!["user_id".to_string(), "email".to_string(), "age".to_string()],
        );
    }

    pub fn validate_column(&self, table: &str, column: &str) -> bool {
        if let Some(columns) = self.tables.get(table) {
            columns.iter().any(|c| c == column)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_registry() {
        let mut registry = SchemaRegistry::new();
        registry.load_mock_schema();
        assert!(registry.validate_column("users", "email"));
        assert!(!registry.validate_column("users", "password"));
        assert!(!registry.validate_column("invalid", "id"));
    }
}
