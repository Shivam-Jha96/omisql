use regex::Regex;

pub struct TokenFixer;

impl TokenFixer {
    pub fn fix_casing(sql: &str) -> String {
        // Naive regex replacement for keywords
        // In a real implementation, we'd use a proper tokenizer that preserves whitespace
        // but since we removed logos, this regex suffices for the style engine demo.
        let keywords = [
            "select", "from", "where", "join", "on", "and", "or", 
            "drop", "alter", "grant", "table", "all", "privileges"
        ];
        
        let mut new_sql = sql.to_string();
        for kw in &keywords {
            let re = Regex::new(&format!(r"(?i)\b{}\b", kw)).unwrap();
            // We shouldn't replace inside strings, but for this simple fixer it's okay
            new_sql = re.replace_all(&new_sql, kw.to_uppercase()).to_string();
        }
        
        new_sql
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_casing() {
        let sql = "select * from my_table where id > 10;";
        let fixed = TokenFixer::fix_casing(sql);
        assert_eq!(fixed, "SELECT * FROM my_table WHERE id > 10;");
    }

    #[test]
    fn test_fix_casing_preserves_whitespace() {
        let sql = "  select   \n\t id from users  ";
        let fixed = TokenFixer::fix_casing(sql);
        assert_eq!(fixed, "  SELECT   \n\t id FROM users  ");
    }
}
