use super::StyleRule;
use regex::Regex;

pub struct TrailingCommaRule;

impl StyleRule for TrailingCommaRule {
    fn apply(&self, sql: &str) -> (String, Vec<String>) {
        let mut current_sql = sql.to_string();
        let mut fixes = Vec::new();
        
        let re = Regex::new(r"(?i),\s+FROM\b").unwrap();
        if re.is_match(&current_sql) {
            current_sql = re.replace_all(&current_sql, " FROM").to_string();
            fixes.push("Removed trailing comma before FROM".to_string());
        }
        
        (current_sql, fixes)
    }
}
