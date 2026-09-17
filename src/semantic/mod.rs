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
    
    pub fn load_from_ddl(&mut self, ddl: &str) {
        let mut current_table = None;
        for line in ddl.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("--") {
                continue;
            }
            if line.to_uppercase().starts_with("CREATE TABLE") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    // CREATE TABLE name (
                    let mut name = parts[2].trim_end_matches('(').to_string();
                    if name.is_empty() && parts.len() > 3 {
                        name = parts[3].trim_end_matches('(').to_string();
                    }
                    current_table = Some(name.clone());
                    self.tables.insert(name, Vec::new());
                }
            } else if line.starts_with(");") || line == ")" {
                current_table = None;
            } else if let Some(ref table_name) = current_table {
                if line.to_uppercase().starts_with("CONSTRAINT") || line.to_uppercase().starts_with("PRIMARY KEY") || line.to_uppercase().starts_with("FOREIGN KEY") || line.to_uppercase().starts_with("UNIQUE") {
                    continue;
                }
                let first_word = line.split_whitespace().next().unwrap_or("").trim_end_matches(',');
                if !first_word.is_empty() && !first_word.starts_with('(') {
                    if let Some(cols) = self.tables.get_mut(table_name) {
                        cols.push(first_word.to_string());
                    }
                }
            }
        }
    }

    pub fn validate_column(&self, table: &str, column: &str) -> bool {
        if let Some(columns) = self.tables.get(table) {
            columns.iter().any(|c| c == column)
        } else {
            false
        }
    }
    
    pub fn find_tables_with_column(&self, column: &str, active_tables: &[String]) -> Vec<String> {
        let mut matches = Vec::new();
        for t in active_tables {
            if self.validate_column(t, column) {
                matches.push(t.clone());
            }
        }
        matches
    }
}
