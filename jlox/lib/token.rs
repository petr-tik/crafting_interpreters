use crate::LoxError;
use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq)]
enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // EOF
    EOF,
}

pub enum Lexeme<'a> {
    Char(char),
    Substr(&'a str),
}

pub struct Token<'a> {
    ty: TokenType,
    lexeme: Option<Lexeme<'a>>,
    loc: u64,
}
type CharIter<'a> = Peekable<Chars<'a>>;

pub struct Scanner<'a> {
    source: CharIter<'a>,
    start: usize,
    current: usize,
    line: u64,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner {
            source: input.chars().peekable(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn tokens(mut self) -> Vec<Token<'a>> {
        let mut tokens = vec![];
        while let Some(c) = self.source.next() {
            let t = match c {
                // Single character tokens
                '(' => Token {
                    ty: TokenType::LeftParen,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },
                ')' => Token {
                    ty: TokenType::RightParen,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },
                '{' => Token {
                    ty: TokenType::LeftBrace,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },
                '}' => Token {
                    ty: TokenType::RightBrace,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },
                ',' => Token {
                    ty: TokenType::Comma,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },
                '.' => Token {
                    ty: TokenType::Dot,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },
                '+' => Token {
                    ty: TokenType::Plus,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },
                '-' => Token {
                    ty: TokenType::Minus,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },
                ';' => Token {
                    ty: TokenType::Semicolon,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },
                '*' => Token {
                    ty: TokenType::Star,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                },

                // Multi-character tokens i.e. operators
                '!' => {
                    if let Some('=') = self.source.next_if_eq(&'=') {
                        Token {
                            ty: TokenType::BangEqual,
                            lexeme: Some(Lexeme::Substr(&"!=")),
                            loc: self.line,
                        }
                    } else {
                        Token {
                            ty: TokenType::Bang,
                            lexeme: Some(Lexeme::Char(c)),
                            loc: self.line,
                        }
                    }
                }
                '=' => {
                    if let Some('=') = self.source.next_if_eq(&'=') {
                        Token {
                            ty: TokenType::EqualEqual,
                            lexeme: Some(Lexeme::Substr(&"==")),
                            loc: self.line,
                        }
                    } else {
                        Token {
                            ty: TokenType::Equal,
                            lexeme: Some(Lexeme::Char(c)),
                            loc: self.line,
                        }
                    }
                }

                _ => todo!("Return a ParserError type"),
            };
            tokens.push(t);
        }
        tokens.push(Token {
            ty: TokenType::EOF,
            lexeme: None,
            loc: self.line,
        });
        tokens
    }
}
