use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\r\f]+")] // Ignore whitespace
pub enum Token {
    #[token("SELECT", ignore(ascii_case))]
    Select,
    #[token("FROM", ignore(ascii_case))]
    From,
    #[token("WHERE", ignore(ascii_case))]
    Where,
    #[token("JOIN", ignore(ascii_case))]
    Join,
    #[token("ON", ignore(ascii_case))]
    On,
    #[token("AND", ignore(ascii_case))]
    And,
    #[token("OR", ignore(ascii_case))]
    Or,
    #[token("GROUP", ignore(ascii_case))]
    Group,
    #[token("BY", ignore(ascii_case))]
    By,
    #[token("DROP", ignore(ascii_case))]
    Drop,
    #[token("ALTER", ignore(ascii_case))]
    Alter,
    #[token("GRANT", ignore(ascii_case))]
    Grant,
    #[token("TABLE", ignore(ascii_case))]
    Table,
    #[token("ALL", ignore(ascii_case))]
    All,
    #[token("PRIVILEGES", ignore(ascii_case))]
    Privileges,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("[0-9]+", |lex| lex.slice().to_string())]
    Number(String),
    #[token("*")]
    Asterisk,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(";")]
    Semicolon,
    #[regex("[=<>!+\\-/]+", |lex| lex.slice().to_string())]
    Operator(String),
}
