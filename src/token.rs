use std::fmt::{self};

use colored::Colorize;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Illegal,
    #[default]
    Eof,

    Ident,
    Int,
    Float,

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
    Ge,
    Le,

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

// 为TokenType 实现fmt方法为了后续
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;
        let str = match self {
            Illegal => "illegal",
            Eof => "EOF",

            Ident => "ident",
            Int => "int",
            Float => "float",

            // operator
            Assign => "=",
            Plus => "+",
            Minus => "-",
            Bang => "!",
            Asterisk => "*",
            Slash => "/",

            Lt => "<",
            Gt => ">",
            Eq => "==",
            NotEq => "!=",
            Ge => ">=",
            Le => "<=",

            Comma => ",",
            Semicolon => ";",

            Lparen => "(",
            Rparen => ")",
            Lbrace => "{",
            Rbrace => "}",

            // key words
            Function => "fn",
            Let => "let",
            If => "if",
            Return => "return",
            Else => "else",
            True => "true",
            False => "false",
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Default)]
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 类型蓝色
        let type_str = format!("{:?}", self.token_type).blue().bold();
        // literal 粉紫色
        let lit_str = self.literal.magenta().bold();

        // ⟮ type ⊢ literal ⟯ （符号本身白色）
        let s = format!(
            "{} {} {} {} {}",
            "⟮".white(),
            type_str,
            "⊢".white(),
            lit_str,
            "⟯".white()
        );

        write!(f, "{}", s)
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
