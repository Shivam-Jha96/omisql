pub mod casing;
pub mod commas;
pub mod fix;

use std::fs::OpenOptions;
use std::io::Write;

pub trait StyleRule {
    fn apply(&self, sql: &str) -> (String, Vec<String>);
}

pub struct StyleEngine {
    rules: Vec<Box<dyn StyleRule>>,
}

impl StyleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Box<dyn StyleRule>) {
        self.rules.push(rule);
    }

    pub fn format_and_log(&self, sql: &str, log_path: &str) -> String {
        let mut current_sql = sql.to_string();
        let mut all_fixes = Vec::new();

        for rule in &self.rules {
            let (new_sql, fixes) = rule.apply(&current_sql);
            current_sql = new_sql;
            all_fixes.extend(fixes);
        }

        if !all_fixes.is_empty() {
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
                for fix in all_fixes {
                    let _ = writeln!(file, "FIX APPLIED: {}", fix);
                }
            }
        }

        current_sql
    }
}
