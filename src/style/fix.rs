use crate::lexer::Token;
use logos::Logos;

pub struct TokenFixer;

impl TokenFixer {
    pub fn fix_casing(sql: &str) -> String {
        let mut lex = Token::lexer(sql);
        let mut new_sql = String::with_capacity(sql.len());
        let mut last_end = 0;
        
        while let Some(res) = lex.next() {
            let span = lex.span();
            new_sql.push_str(&sql[last_end..span.start]); // Push whitespace/untokenized text
            
            if let Ok(token) = res {
                match token {
                    Token::Select | Token::From | Token::Where | Token::Join | 
                    Token::On | Token::And | Token::Or | Token::Drop | Token::Alter | 
                    Token::Grant | Token::Table | Token::All | Token::Privileges => {
                        // These are keywords, we should uppercase them
                        new_sql.push_str(&lex.slice().to_uppercase());
                    },
                    _ => {
                        new_sql.push_str(lex.slice());
                    }
                }
            } else {
                new_sql.push_str(lex.slice());
            }
            
            last_end = span.end;
        }
        
        new_sql.push_str(&sql[last_end..]);
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
