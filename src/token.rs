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
    Eq,
    NotEq,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // key words
    Function,
    Let,
    If,
    Return,
    Else,
    True,
    False,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    // new function for char
    pub fn new_with_char(token_type: TokenType, literal: char) -> Token {
        Token {
            token_type: token_type,
            literal: literal.to_string(),
        }
    }

    // new function for string
    pub fn new_with_string(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type: token_type,
            literal: literal,
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
        "if" => return TokenType::If,
        "else" => return TokenType::Else,
        "return" => return TokenType::Return,
        "true" => return TokenType::True,
        "false" => return TokenType::False,
        _ => return TokenType::Ident,
    }
}
