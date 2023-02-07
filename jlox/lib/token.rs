#[derive(PartialEq)]
enum SingleC {
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
}

#[derive(PartialEq)]
enum NonSignleC {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(PartialEq)]
enum Lit {
    Identifier,
    String,
    Number,
}

#[derive(PartialEq)]
enum KeyWord {
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
}

#[derive(PartialEq)]
enum TokenType {
    SingleChar(SingleC),
    OneOrTwoChar(NonSignleC),
    Literal(Lit),
    Keyword(KeyWord),
    EOF,
}

pub struct Token<'a> {
    ty: TokenType,
    lexeme: &'a str,
    loc: usize,
}

struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: u64,
}

impl<'a> Scanner<'a> {
    fn is_at_end(&self) -> bool {
        self.current > self.source.len()
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
