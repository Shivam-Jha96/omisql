pub mod security;
pub mod cost;
pub mod dbt;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TableDef {
    pub columns: Vec<String>,
    pub pii_columns: Vec<String>,
    pub partition_column: Option<String>,
}

pub struct SchemaRegistry {
    pub tables: HashMap<String, TableDef>,
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
            TableDef {
                columns: vec!["user_id".to_string(), "email".to_string(), "age".to_string()],
                pii_columns: vec![],
                partition_column: None,
            },
        );
    }
    
    pub fn load_from_dbt_manifest(&mut self, path: &str) -> Result<(), anyhow::Error> {
        let manifest = dbt::DbtManifest::load_from_file(path)?;
        
        let mut process_nodes = |nodes: &HashMap<String, dbt::DbtNode>| {
            for (_, node) in nodes {
                // In dbt, nodes include models, seeds, snapshots, etc.
                if node.resource_type == "model" || node.resource_type == "source" || node.resource_type == "seed" {
                    let mut table_def = TableDef {
                        columns: Vec::new(),
                        pii_columns: Vec::new(),
                        partition_column: None,
                    };
                    
                    for (col_name, col_def) in &node.columns {
                        table_def.columns.push(col_name.clone());
                        
                        if let Some(pii) = col_def.meta.get("pii") {
                            if pii.as_bool() == Some(true) {
                                table_def.pii_columns.push(col_name.clone());
                            }
                        }
                    }
                    
                    // Note: Dbt doesn't have a standard partition_column meta, but users could define it
                    if let Some(partition) = node.meta.get("partition_column") {
                        if let Some(p_str) = partition.as_str() {
                            table_def.partition_column = Some(p_str.to_string());
                        }
                    }

                    // For now, use the node name as the table name
                    self.tables.insert(node.name.clone(), table_def);
                }
            }
        };

        process_nodes(&manifest.nodes);
        process_nodes(&manifest.sources);

        Ok(())
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
                    self.tables.insert(name, TableDef {
                        columns: Vec::new(),
                        pii_columns: Vec::new(),
                        partition_column: None,
                    });
                }
            } else if line.starts_with(");") || line == ")" {
                current_table = None;
            } else if let Some(ref table_name) = current_table {
                if line.to_uppercase().starts_with("CONSTRAINT") || line.to_uppercase().starts_with("PRIMARY KEY") || line.to_uppercase().starts_with("FOREIGN KEY") || line.to_uppercase().starts_with("UNIQUE") {
                    continue;
                }
                let first_word = line.split_whitespace().next().unwrap_or("").trim_end_matches(',');
                if !first_word.is_empty() && !first_word.starts_with('(') {
                    if let Some(table_def) = self.tables.get_mut(table_name) {
                        let col_name = first_word.to_string();
                        table_def.columns.push(col_name.clone());
                        
                        if line.contains("COMMENT '@PII'") || line.contains("COMMENT '@pii'") {
                            table_def.pii_columns.push(col_name.clone());
                        }
                        if line.contains("COMMENT '@PARTITION'") || line.contains("COMMENT '@partition'") {
                            table_def.partition_column = Some(col_name);
                        }
                    }
                }
            }
        }
    }

    pub fn validate_column(&self, table: &str, column: &str) -> bool {
        if let Some(table_def) = self.tables.get(table) {
            table_def.columns.iter().any(|c| c == column)
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
