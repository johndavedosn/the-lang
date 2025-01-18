#![allow(unused)]
use logos::Logos;
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] 
// I don't know why I can't set the int and float ones to
// an int or float type.
pub enum Token {
    #[token("else", |lex| lex.slice().to_string())]
    #[token("if", |lex| lex.slice().to_string())]
    #[token("for", |lex| lex.slice().to_string())]
    #[token("while", |lex| lex.slice().to_string())]
    #[token("let", |lex| lex.slice().to_string())]
    #[token("func", |lex| lex.slice().to_string())]
    #[token("return", |lex| lex.slice().to_string())]
    Keyword(String),
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex(r"(\+|\-)?\d+", |lex| lex.slice().to_string())]
    Int(String),
    #[regex(r"(\+|\-)?\d+.\d+", |lex| lex.slice().to_string())]
    Float(String),
    #[token["true", |lex| lex.slice().to_string()]]
    #[token["false", |lex| lex.slice().to_string()]]
    Boolean(String),
}

pub fn lex(input: &str) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Token::lexer(input);
    while let Some(token) = lexer.next() {
        match token {
            Ok(tk) => {
                tokens.push(tk);
            },
            Err(_) => {
                // I still need to figure out what to do here.
            }
        }
    }
    tokens
}