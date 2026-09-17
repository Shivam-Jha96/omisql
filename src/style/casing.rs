use super::StyleRule;
use regex::Regex;

pub struct KeywordCasingRule;

impl StyleRule for KeywordCasingRule {
    fn apply(&self, sql: &str) -> (String, Vec<String>) {
        let mut current_sql = sql.to_string();
        let mut fixes = Vec::new();
        
        let keywords = ["select", "from", "where", "join", "on", "and", "or", "group by"];
        
        for kw in keywords {
            let upper_kw = kw.to_uppercase();
            let re = Regex::new(&format!(r"(?i)\b{}\b", kw)).unwrap();
            
            let has_matches = re.find_iter(&current_sql).any(|m| m.as_str() != upper_kw);
            if has_matches {
                current_sql = re.replace_all(&current_sql, upper_kw.as_str()).to_string();
                fixes.push(format!("Capitalized keyword '{}'", upper_kw));
            }
        }
        
        (current_sql, fixes)
    }
}
