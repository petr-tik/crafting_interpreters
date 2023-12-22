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

pub struct Token<'a> {
    ty: TokenType,
    lexeme: &'a str,
    loc: usize,
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
        loop {
            //     let t = self.scan_token();
            //     tokens.push(t);
            //     if t.ty == TokenType::EOF {
            //         break;
            //     }
        }
        tokens
    }
}
