use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore whitespace
pub enum Token {
    #[token("SELECT", ignore(ascii_case))]
    Select,
    #[token("FROM", ignore(ascii_case))]
    From,
    #[token("WHERE", ignore(ascii_case))]
    Where,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("[0-9]+")]
    Number,
    #[token("*")]
    Asterisk,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[regex("[=<>!]+")]
    Operator,
}
