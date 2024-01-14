use crate::LoxError;
use crate::error::LexError;
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
    Comment,
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
    line: u64,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner {
            source: input.chars().peekable(),
            line: 0,
        }
    }

    pub fn tokens(mut self) -> Vec<Token<'a>> {
        let mut tokens = vec![];
        let mut errors = vec![];
        while let Some(c) = self.source.next() {
            let t: Option<Token> = match c {
                // Single character tokens
                '(' => Some(Token {
                    ty: TokenType::LeftParen,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),
                ')' => Some(Token {
                    ty: TokenType::RightParen,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),
                '{' => Some(Token {
                    ty: TokenType::LeftBrace,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),
                '}' => Some(Token {
                    ty: TokenType::RightBrace,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),
                ',' => Some(Token {
                    ty: TokenType::Comma,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),
                '.' => Some(Token {
                    ty: TokenType::Dot,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),
                '+' => Some(Token {
                    ty: TokenType::Plus,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),
                '-' => Some(Token {
                    ty: TokenType::Minus,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),
                ';' => Some(Token {
                    ty: TokenType::Semicolon,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),
                '*' => Some(Token {
                    ty: TokenType::Star,
                    lexeme: Some(Lexeme::Char(c)),
                    loc: self.line,
                }),

                // Multi-character tokens i.e. operators
                '!' => {
                    if let Some('=') = self.source.next_if_eq(&'=') {
                        Some(Token {
                            ty: TokenType::BangEqual,
                            lexeme: Some(Lexeme::Substr(&"!=")),
                            loc: self.line,
                        })
                    } else {
                        Some(Token {
                            ty: TokenType::Bang,
                            lexeme: Some(Lexeme::Char(c)),
                            loc: self.line,
                        })
                    }
                }
                '=' => {
                    if let Some('=') = self.source.next_if_eq(&'=') {
                        Some(Token {
                            ty: TokenType::EqualEqual,
                            lexeme: Some(Lexeme::Substr(&"==")),
                            loc: self.line,
                        })
                    } else {
                        Some(Token {
                            ty: TokenType::Equal,
                            lexeme: Some(Lexeme::Char(c)),
                            loc: self.line,
                        })
                    }
                }
                '>' => {
                    if let Some('=') = self.source.next_if_eq(&'=') {
                        Some(Token {
                            ty: TokenType::GreaterEqual,
                            lexeme: Some(Lexeme::Substr(&"==")),
                            loc: self.line,
                        })
                    } else {
                        Some(Token {
                            ty: TokenType::Greater,
                            lexeme: Some(Lexeme::Char(c)),
                            loc: self.line,
                        })
                    }
                }
                '<' => {
                    if let Some('=') = self.source.next_if_eq(&'=') {
                        Some(Token {
                            ty: TokenType::LessEqual,
                            lexeme: Some(Lexeme::Substr(&"==")),
                            loc: self.line,
                        })
                    } else {
                        Some(Token {
                            ty: TokenType::Less,
                            lexeme: Some(Lexeme::Char(c)),
                            loc: self.line,
                        })
                    }
                }
                '/' => {
                    if let Some('/') = self.source.next_if_eq(&'/') {
                        // REVIEW can I reuse the existing self.source iterator?
                        let mut comment_iter = self.source.clone().take_while(|c| c != &'\n');
                        while let Some(_) = comment_iter.next() {}
                        Some(Token {
                            ty: TokenType::Comment,
                            // TODO return the lexeme from all the characters until the end of the line
                            lexeme: None,
                            loc: self.line,
                        })
                    } else {
                        Some(Token {
                            ty: TokenType::Slash,
                            lexeme: Some(Lexeme::Char(c)),
                            loc: self.line,
                        })
                    }
                }
                ' ' | '\r' | '\t' => None,
                '\n' => {
                    self.line += 1;
                    None
                }
                _ => {
                    let lex_err = LexError {error_c : c, line : self.line};
                    errors.push(LoxError::LexerError(lex_err));
                    None
                }
            };
            if let Some(t) = t {
                tokens.push(t)
            }
        }
        tokens.push(Token {
            ty: TokenType::EOF,
            lexeme: None,
            loc: self.line,
        });
        // TODO come up with an error handling strategy
        dbg!(&errors);
        assert!(errors.is_empty(), "Failed to parse some tokens");
        tokens
    }
}
