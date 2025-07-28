#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident,
    Int,

    // operator
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // key words
    Function,
    Let,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: char) -> Token {
        Token {
            token_type: token_type,
            literal: literal.to_string(),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.literal == other.literal && self.token_type == other.token_type
    }
}

pub fn lookup_ident(ident: String) -> TokenType {
    match ident.as_str() {
        "fn" => return TokenType::Function,
        "let" => return TokenType::Let,
        _ => return TokenType::Ident,
    }
}
