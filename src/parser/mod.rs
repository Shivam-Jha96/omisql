use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    pub columns: Vec<String>,
    pub table: String,
}

pub fn parse_select(tokens: &[Token]) -> Result<SelectStatement, String> {
    let mut columns = Vec::new();
    let table;
    let mut iter = tokens.iter();

    // Expect SELECT
    match iter.next() {
        Some(Token::Select) => {}
        _ => return Err("Expected SELECT statement".to_string()),
    }

    // Parse columns
    loop {
        match iter.next() {
            Some(Token::Identifier(name)) => {
                columns.push(name.clone());
            }
            Some(Token::Asterisk) => {
                columns.push("*".to_string());
            }
            Some(Token::Comma) => continue,
            Some(Token::From) => break,
            Some(tok) => return Err(format!("Unexpected token in SELECT clause: {:?}", tok)),
            None => return Err("Unexpected end of tokens".to_string()),
        }
    }

    // Parse table
    match iter.next() {
        Some(Token::Identifier(name)) => {
            table = name.clone();
        }
        Some(tok) => return Err(format!("Expected table name, found: {:?}", tok)),
        None => return Err("Unexpected end of tokens".to_string()),
    }

    // Optional semicolon
    if let Some(Token::Semicolon) = iter.as_slice().first() {
        iter.next();
    }

    Ok(SelectStatement { columns, table })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    #[test]
    fn test_parse_select_asterisk() {
        let tokens = vec![
            Token::Select,
            Token::Asterisk,
            Token::From,
            Token::Identifier("table_name".to_string()),
            Token::Semicolon,
        ];
        
        let ast = parse_select(&tokens).unwrap();
        assert_eq!(ast.columns, vec!["*".to_string()]);
        assert_eq!(ast.table, "table_name".to_string());
    }

    #[test]
    fn test_parse_select_columns() {
        let tokens = vec![
            Token::Select,
            Token::Identifier("id".to_string()),
            Token::Comma,
            Token::Identifier("name".to_string()),
            Token::From,
            Token::Identifier("table_name".to_string()),
        ];
        
        let ast = parse_select(&tokens).unwrap();
        assert_eq!(
            ast.columns,
            vec!["id".to_string(), "name".to_string()]
        );
        assert_eq!(ast.table, "table_name".to_string());
    }

    #[test]
    fn test_parse_error() {
        let tokens = vec![Token::From];
        let err = parse_select(&tokens).unwrap_err();
        assert_eq!(err, "Expected SELECT statement");
    }
}
