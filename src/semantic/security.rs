use crate::parser::Statement;
use crate::semantic::SchemaRegistry;

pub enum SecurityIssue {
    Error(String),
    Warning(String),
}

pub fn check_security_rules(stmt: &Statement, registry: &SchemaRegistry) -> Vec<SecurityIssue> {
    let mut issues = Vec::new();
    
    match stmt {
        Statement::Select(select) => {
            // UnmaskedPIISelect
            for col in &select.columns {
                let tables_to_check = if let Some(ref t) = col.table {
                    vec![t.clone()]
                } else {
                    let mut active_tables = vec![select.table.clone()];
                    for join in &select.joins {
                        active_tables.push(join.table.clone());
                    }
                    registry.find_tables_with_column(&col.name, &active_tables)
                };

                for t in tables_to_check {
                    if let Some(table_def) = registry.tables.get(&t) {
                        if table_def.pii_columns.contains(&col.name) {
                            issues.push(SecurityIssue::Error(format!(
                                "UnmaskedPIISelect: Selecting PII column '{}.{}' without masking.", t, col.name
                            )));
                        }
                    }
                }
            }
        }
        Statement::Drop(drop) => {
            issues.push(SecurityIssue::Error(format!(
                "DangerousMigration: Destructive DROP TABLE '{}' detected.", drop.table
            )));
        }
        Statement::Alter(alter) => {
            issues.push(SecurityIssue::Warning(format!(
                "DangerousMigration: ALTER TABLE '{}' detected. Review for destructive operations.", alter.table
            )));
        }
        Statement::Grant(grant) => {
            if grant.all_privileges {
                issues.push(SecurityIssue::Error(
                    "UnsafeGrant: GRANT ALL PRIVILEGES is overly permissive and prohibited.".to_string()
                ));
            }
        }
        Statement::Ignored => {}
    }
    
    issues
}
