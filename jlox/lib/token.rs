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

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: u64,
}

impl<'a> Scanner<'a> {
    fn is_at_end(&self) -> bool {
        self.current > self.source.len()
    }

    pub fn new(input: &'a str) -> Self {
        Scanner {
            source: input,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn tokens(mut self) -> Vec<Token<'a>> {
        let mut tokens = vec![];
        while !self.is_at_end() {
            // let t = self.scan_token();
            // if t.ty == TokenType::EOF {
            //     break;
            // }
            self.current += 1;
        }
        tokens.push(Token {
            ty: TokenType::EOF,
            lexeme: None,
            loc: self.line,
        });
        tokens
    }
}
