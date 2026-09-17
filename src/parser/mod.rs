use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnRef {
    pub table: Option<String>,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Column(ColumnRef),
    Number(String),
    BinaryOp {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct JoinClause {
    pub table: String,
    pub on: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SelectStatement {
    pub columns: Vec<ColumnRef>,
    pub table: String,
    pub joins: Vec<JoinClause>,
    pub where_clause: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DropStatement {
    pub table: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AlterStatement {
    pub table: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GrantStatement {
    pub all_privileges: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Select(SelectStatement),
    Drop(DropStatement),
    Alter(AlterStatement),
    Grant(GrantStatement),
}

pub fn parse_statement(tokens: &[Token]) -> Result<Statement, String> {
    if tokens.is_empty() {
        return Err("Empty statement".to_string());
    }
    match tokens[0] {
        Token::Select => Ok(Statement::Select(parse_select(tokens)?)),
        Token::Drop => Ok(Statement::Drop(parse_drop(tokens)?)),
        Token::Alter => Ok(Statement::Alter(parse_alter(tokens)?)),
        Token::Grant => Ok(Statement::Grant(parse_grant(tokens)?)),
        _ => Err("Unsupported statement type".to_string()),
    }
}

fn parse_drop(tokens: &[Token]) -> Result<DropStatement, String> {
    let mut iter = tokens.iter().peekable();
    iter.next(); // Consume DROP
    if let Some(Token::Table) = iter.next() {
        if let Some(Token::Identifier(name)) = iter.next() {
            return Ok(DropStatement { table: name.clone() });
        }
    }
    Err("Invalid DROP statement".to_string())
}

fn parse_alter(tokens: &[Token]) -> Result<AlterStatement, String> {
    let mut iter = tokens.iter().peekable();
    iter.next(); // Consume ALTER
    if let Some(Token::Table) = iter.next() {
        if let Some(Token::Identifier(name)) = iter.next() {
            return Ok(AlterStatement { table: name.clone() });
        }
    }
    Err("Invalid ALTER statement".to_string())
}

fn parse_grant(tokens: &[Token]) -> Result<GrantStatement, String> {
    let mut iter = tokens.iter().peekable();
    iter.next(); // Consume GRANT
    let mut all_privileges = false;
    if let Some(Token::All) = iter.next() {
        if let Some(Token::Privileges) = iter.peek() {
            all_privileges = true;
        }
    }
    Ok(GrantStatement { all_privileges })
}

pub fn parse_select(tokens: &[Token]) -> Result<SelectStatement, String> {
    let mut columns = Vec::new();
    let table;
    let mut joins = Vec::new();
    let mut where_clause = None;

    let mut iter = tokens.iter().peekable();

    // Expect SELECT
    match iter.next() {
        Some(Token::Select) => {}
        _ => return Err("Expected SELECT statement".to_string()),
    }

    // Parse columns
    loop {
        match iter.next() {
            Some(Token::Identifier(name)) => {
                let mut col_name = name.clone();
                let mut table_name = None;
                if let Some(&&Token::Dot) = iter.peek() {
                    iter.next(); // consume dot
                    if let Some(Token::Identifier(sub_name)) = iter.next() {
                        table_name = Some(col_name);
                        col_name = sub_name.clone();
                    } else {
                        return Err("Expected identifier after dot".to_string());
                    }
                }
                columns.push(ColumnRef { table: table_name, name: col_name });
            }
            Some(Token::Asterisk) => {
                columns.push(ColumnRef { table: None, name: "*".to_string() });
            }
            Some(Token::Comma) => continue,
            Some(Token::From) => break,
            Some(tok) => return Err(format!("Unexpected token in SELECT clause: {:?}", tok)),
            None => return Err("Unexpected end of tokens".to_string()),
        }
    }

    // Parse main table
    match iter.next() {
        Some(Token::Identifier(name)) => {
            table = name.clone();
        }
        Some(tok) => return Err(format!("Expected table name, found: {:?}", tok)),
        None => return Err("Unexpected end of tokens".to_string()),
    }

    // Parse optional clauses
    while let Some(&peek_tok) = iter.peek() {
        match peek_tok {
            Token::Join => {
                iter.next(); // Consume JOIN
                let join_table = match iter.next() {
                    Some(Token::Identifier(name)) => name.clone(),
                    _ => return Err("Expected table name after JOIN".to_string()),
                };
                
                match iter.next() {
                    Some(Token::On) => {}
                    _ => return Err("Expected ON after JOIN table".to_string()),
                }

                let on_condition = parse_expression(&mut iter)?;
                joins.push(JoinClause {
                    table: join_table,
                    on: on_condition,
                });
            }
            Token::Where => {
                iter.next(); // Consume WHERE
                where_clause = Some(parse_expression(&mut iter)?);
            }
            Token::Semicolon => {
                iter.next(); // Consume Semicolon
                break;
            }
            _ => {
                break;
            }
        }
    }

    Ok(SelectStatement {
        columns,
        table,
        joins,
        where_clause,
    })
}

// Simple expression parser
fn parse_expression<'a, I>(iter: &mut std::iter::Peekable<I>) -> Result<Expression, String>
where
    I: Iterator<Item = &'a Token>,
{
    let left = match iter.next() {
        Some(Token::Identifier(name)) => {
            let mut col_name = name.clone();
            let mut table_name = None;
            if let Some(&&Token::Dot) = iter.peek() {
                iter.next(); // consume dot
                if let Some(Token::Identifier(sub_name)) = iter.next() {
                    table_name = Some(col_name);
                    col_name = sub_name.clone();
                } else {
                    return Err("Expected identifier after dot".to_string());
                }
            }
            Expression::Column(ColumnRef { table: table_name, name: col_name })
        }
        Some(Token::Number(val)) => Expression::Number(val.clone()),
        _ => return Err("Expected identifier or number in expression".to_string()),
    };

    let operator = match iter.peek() {
        Some(&Token::Operator(op)) => {
            let op_str = op.clone();
            iter.next(); // consume op
            op_str
        }
        Some(&Token::Asterisk) => { iter.next(); "*".to_string() }
        Some(&Token::And) => { iter.next(); "AND".to_string() }
        Some(&Token::Or) => { iter.next(); "OR".to_string() }
        _ => return Ok(left), // Single token expression
    };

    let right = parse_expression(iter)?;

    Ok(Expression::BinaryOp {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(ast.columns[0].name, "*");
        assert_eq!(ast.table, "table_name".to_string());
    }

    #[test]
    fn test_parse_where() {
        let tokens = vec![
            Token::Select,
            Token::Identifier("users".to_string()),
            Token::Dot,
            Token::Identifier("id".to_string()),
            Token::From,
            Token::Identifier("users".to_string()),
            Token::Where,
            Token::Identifier("age".to_string()),
            Token::Operator(">".to_string()),
            Token::Number("18".to_string()),
        ];
        
        let ast = parse_select(&tokens).unwrap();
        assert_eq!(ast.table, "users".to_string());
        assert_eq!(ast.columns[0].table, Some("users".to_string()));
        assert_eq!(ast.columns[0].name, "id".to_string());
        assert!(ast.where_clause.is_some());
    }
}
