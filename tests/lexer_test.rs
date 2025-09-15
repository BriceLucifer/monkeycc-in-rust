#[cfg(test)]
mod lexer_test {
    use monkeycc::lexer::Lexer;
    use monkeycc::token::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = r"let five = 5;
            let ten = 10;
            let add = fn(x,y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            ";

        let tests = vec![
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "five".to_string(),
            },
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Function,
                literal: "fn".to_string(),
            },
            Token {
                token_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "y".to_string(),
            },
            Token {
                token_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "y".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "result".to_string(),
            },
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "five".to_string(),
            },
            Token {
                token_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Bang,
                literal: "!".to_string(),
            },
            Token {
                token_type: TokenType::Minus,
                literal: "-".to_string(),
            },
            Token {
                token_type: TokenType::Slash,
                literal: "/".to_string(),
            },
            Token {
                token_type: TokenType::Asterisk,
                literal: "*".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::Lt,
                literal: "<".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::Gt,
                literal: ">".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::If,
                literal: "if".to_string(),
            },
            Token {
                token_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::Lt,
                literal: "<".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                token_type: TokenType::True,
                literal: "true".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::Else,
                literal: "else".to_string(),
            },
            Token {
                token_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                token_type: TokenType::False,
                literal: "false".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::Eq,
                literal: "==".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::NotEq,
                literal: "!=".to_string(),
            },
            Token {
                token_type: TokenType::Int,
                literal: "9".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
        ];

        // 创建Lexer
        let mut l: Lexer = Lexer::new(input);

        // 循环递归
        for tt in tests {
            let tok = l.next_token();
            // 对比
            // 首先两个type 要实现PartialEq trait
            assert_eq!(tt, tok);
        }
    }
}
