use serde::Serialize;
use std::io::{self, Read};

#[derive(Serialize)]
pub struct LintIssue {
    pub rule_name: String,
    pub severity: String,
    pub message: String,
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    
    let ast: serde_json::Value = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(_) => return,
    };
    
    let mut issues = Vec::new();
    
    if let Some(select) = ast.get("Select") {
        if let Some(columns) = select.get("columns") {
            if let Some(cols_array) = columns.as_array() {
                for col in cols_array {
                    if let Some(name) = col.get("name") {
                        if name == "*" {
                            issues.push(LintIssue {
                                rule_name: "NoSelectStar".to_string(),
                                severity: "Warning".to_string(),
                                message: "Avoid using SELECT *, specify columns explicitly.".to_string(),
                            });
                            break;
                        }
                    }
                }
            }
        }
        
        if let Some(joins) = select.get("joins") {
            if let Some(joins_array) = joins.as_array() {
                if joins_array.len() > 3 {
                    issues.push(LintIssue {
                        rule_name: "TooManyJoins".to_string(),
                        severity: "Warning".to_string(),
                        message: format!("Query has {} joins. Consider simplifying for performance.", joins_array.len()),
                    });
                }
            }
        }
    }
    
    if !issues.is_empty() {
        if let Ok(json) = serde_json::to_string(&issues) {
            println!("{}", json);
        }
    }
}
