
use sqlparser::parser::{Parser, ParserError};
use sqlparser::ast::{self as sqlast, SelectItem, Expr, JoinOperator, TableFactor};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ColumnRef {
    pub table: Option<String>,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Expression {
    Column(ColumnRef),
    Number(String),
    BinaryOp {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct JoinClause {
    pub table: String,
    pub on: Expression,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct SelectStatement {
    pub columns: Vec<ColumnRef>,
    pub table: String,
    pub joins: Vec<JoinClause>,
    pub where_clause: Option<Expression>,
    pub group_by: Option<Vec<ColumnRef>>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct DropStatement {
    pub table: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlterStatement {
    pub table: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct GrantStatement {
    pub all_privileges: bool,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Statement {
    Select(SelectStatement),
    Drop(DropStatement),
    Alter(AlterStatement),
    Grant(GrantStatement),
    Ignored,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

use std::sync::OnceLock;
use regex::Regex;

static OWNER_RE: OnceLock<Regex> = OnceLock::new();
static COPY_RE: OnceLock<Regex> = OnceLock::new();
static UNSUPPORTED_RE: OnceLock<Regex> = OnceLock::new();

pub fn preprocess_sql(sql: &str) -> String {
    let owner_re = OWNER_RE.get_or_init(|| Regex::new(r"(?i)ALTER\s+[a-z_]+\s+[^;]*?OWNER\s+TO\s+[^;]*?;").unwrap());
    let copy_re = COPY_RE.get_or_init(|| Regex::new(r"(?i)COPY\s+[\s\S]*?\s+FROM\s+stdin.*?;[\s\S]*?\n\\\.").unwrap());
    let unsupported_re = UNSUPPORTED_RE.get_or_init(|| Regex::new(r"(?i)CREATE\s+(?:SEQUENCE|AGGREGATE|TRIGGER|DOMAIN|TYPE)\s+[^;]*?;").unwrap());

    let sql_owner = owner_re.replace_all(sql, |caps: &regex::Captures| {
        let num_newlines = caps[0].matches('\n').count();
        "\n".repeat(num_newlines)
    });
    
    let sql_copy = copy_re.replace_all(&sql_owner, |caps: &regex::Captures| {
        let num_newlines = caps[0].matches('\n').count();
        "\n".repeat(num_newlines)
    });

    let sql_unsupported = unsupported_re.replace_all(&sql_copy, |caps: &regex::Captures| {
        let num_newlines = caps[0].matches('\n').count();
        "\n".repeat(num_newlines)
    });

    sql_unsupported.into_owned()
}

pub fn parse_sql(sql: &str, dialect_name: &str) -> Result<Statement, ParseError> {
    let dialect: Box<dyn sqlparser::dialect::Dialect> = match dialect_name.to_lowercase().as_str() {
        "snowflake" => Box::new(sqlparser::dialect::SnowflakeDialect {}),
        "generic" => Box::new(sqlparser::dialect::GenericDialect {}),
        _ => Box::new(sqlparser::dialect::PostgreSqlDialect {}),
    };
    let preprocessed = preprocess_sql(sql);
    let ast_list = match Parser::parse_sql(dialect.as_ref(), &preprocessed) {
        Ok(ast) => ast,
        Err(e) => {
            let mut line = 1;
            let mut col = 1;
            if let ParserError::ParserError(ref msg) = e {
                if let Some(pos) = msg.find(" at Line: ") {
                    let parts: Vec<&str> = msg[pos..].split(',').collect();
                    if parts.len() >= 2 {
                        if let Ok(l) = parts[0].replace(" at Line: ", "").trim().parse() {
                            line = l;
                        }
                        if let Ok(c) = parts[1].replace(" Column: ", "").trim().parse() {
                            col = c;
                        }
                    }
                }
            }
            return Err(ParseError { message: e.to_string(), line, col });
        }
    };

    if ast_list.is_empty() {
        return Err(ParseError { message: "Empty statement".to_string(), line: 1, col: 1 });
    }
    
    Ok(convert_statement(&ast_list[0]))
}

fn convert_statement(stmt: &sqlast::Statement) -> Statement {
    match stmt {
        sqlast::Statement::Query(query) => {
            if let sqlast::SetExpr::Select(select) = &*query.body {
                let columns = convert_select_items(&select.projection);
                
                let mut table = String::new();
                let mut joins = Vec::new();
                
                if let Some(from) = select.from.first() {
                    table = extract_table_name(&from.relation);
                    
                    for join in &from.joins {
                        let join_table = extract_table_name(&join.relation);
                        let on_expr = match &join.join_operator {
                            JoinOperator::Inner(sqlast::JoinConstraint::On(expr))
                            | JoinOperator::LeftOuter(sqlast::JoinConstraint::On(expr))
                            | JoinOperator::RightOuter(sqlast::JoinConstraint::On(expr))
                            | JoinOperator::FullOuter(sqlast::JoinConstraint::On(expr)) => {
                                convert_expr(expr)
                            }
                            _ => Expression::Number("1".to_string()), // fallback
                        };
                        joins.push(JoinClause { table: join_table, on: on_expr });
                    }
                }
                
                let where_clause = select.selection.as_ref().map(convert_expr);
                
                // Note: group_by struct might differ slightly between sqlparser versions
                let group_by = match &select.group_by {
                    sqlast::GroupByExpr::Expressions(exprs, _) if !exprs.is_empty() => {
                        let mut gb = Vec::new();
                        for e in exprs {
                            if let Expression::Column(c) = convert_expr(e) {
                                gb.push(c);
                            }
                        }
                        Some(gb)
                    }
                    _ => None,
                };
                
                Statement::Select(SelectStatement {
                    columns,
                    table,
                    joins,
                    where_clause,
                    group_by,
                })
            } else {
                Statement::Ignored
            }
        }
        sqlast::Statement::Drop { object_type, names, .. } => {
            if matches!(object_type, sqlast::ObjectType::Table) && !names.is_empty() {
                Statement::Drop(DropStatement { table: names[0].to_string() })
            } else {
                Statement::Ignored
            }
        }
        sqlast::Statement::AlterTable(alter) => {
            Statement::Alter(AlterStatement { table: alter.name.to_string() })
        }
        sqlast::Statement::Grant(grant) => {
            let all_privileges = matches!(grant.privileges, sqlast::Privileges::All { .. });
            Statement::Grant(GrantStatement { all_privileges })
        }
        _ => Statement::Ignored,
    }
}

fn extract_table_name(relation: &TableFactor) -> String {
    match relation {
        TableFactor::Table { name, .. } => name.to_string(),
        _ => "unknown".to_string(),
    }
}

fn convert_select_items(items: &[SelectItem]) -> Vec<ColumnRef> {
    let mut cols = Vec::new();
    for item in items {
        match item {
            SelectItem::UnnamedExpr(expr) | SelectItem::ExprWithAlias { expr, .. } | SelectItem::ExprWithAliases { expr, .. } => {
                if let Expression::Column(c) = convert_expr(expr) {
                    cols.push(c);
                } else {
                    cols.push(ColumnRef { table: None, name: "function_or_expr".to_string() });
                }
            }
            SelectItem::Wildcard(_) => {
                cols.push(ColumnRef { table: None, name: "*".to_string() });
            }
            SelectItem::QualifiedWildcard(name, _) => {
                cols.push(ColumnRef { table: Some(name.to_string()), name: "*".to_string() });
            }
        }
    }
    cols
}

fn convert_expr(expr: &Expr) -> Expression {
    match expr {
        Expr::Identifier(ident) => Expression::Column(ColumnRef { table: None, name: ident.value.clone() }),
        Expr::CompoundIdentifier(idents) => {
            if idents.len() >= 2 {
                let t = idents[idents.len()-2].value.clone();
                let c = idents[idents.len()-1].value.clone();
                Expression::Column(ColumnRef { table: Some(t), name: c })
            } else {
                Expression::Column(ColumnRef { table: None, name: idents[0].value.clone() })
            }
        }
        Expr::Value(val) => Expression::Number(val.to_string()),
        Expr::BinaryOp { left, op, right } => {
            Expression::BinaryOp {
                left: Box::new(convert_expr(left)),
                operator: op.to_string(),
                right: Box::new(convert_expr(right)),
            }
        }
        _ => Expression::Number("0".to_string()), // fallback
    }
}
