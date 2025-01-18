#![allow(unused)]
use logos::{Lexer, Logos};
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] 
pub enum Token {
    #[token("else", which_keyowrd)]
    #[token("if", which_keyowrd)]
    #[token("for", which_keyowrd)]
    #[token("while", which_keyowrd)]
    #[token("let", which_keyowrd)]
    #[token("func", which_keyowrd)]
    #[token("return", which_keyowrd)]
    Keyword(Keywords),
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("0x[0-9a-f_]+", |lex| parse_int(&lex.slice()[2..], 16))]
    #[regex("0b[01_]+", |lex| parse_int(&lex.slice()[2..], 2))]
    #[regex("0o[0-7_]+", |lex| parse_int(&lex.slice()[2..], 8))]
    #[regex(r"\d+[_\d]*", |lex| parse_int(lex.slice(), 10))]
    Int(isize),
    #[regex(r"\d+\.\d+", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Boolean(bool),
    
}
#[derive(Debug, PartialEq)]
pub enum Keywords {
    If,
    Else,
    For,
    While,
    Func,
    Return,
    Invalid
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
fn which_keyowrd<'s>(lex: &mut Lexer<'s, Token>) -> Keywords {
    match lex.slice() {
        "if" => Keywords::If,
        "else" => Keywords::Else,
        "for" => Keywords::For,
        "while" => Keywords::While,
        "func" => Keywords::Func,
        "return" => Keywords::Return,
        _ => Keywords::Invalid
    }
}
fn parse_int(val: &str, radix: u32) -> isize {
    match isize::from_str_radix(&val.replace('_', ""), radix) {
        Ok(n) => n,
        Err(err) => panic!("Failed to parse base-{radix} integer {val:?}: {err:?}")
    }
}