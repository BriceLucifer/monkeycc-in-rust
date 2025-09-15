use crate::{
    ast::{Expr, Ident, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    // lexer
    l: Lexer,
    // error massage collect
    errors: Vec<String>,
    // 当前的token
    cur_token: Token,
    // 下一个预测的token
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        // 初始化
        let mut p: Parser = Parser {
            l: lexer,
            errors: Vec::new(),
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        p.next_token();
        p.next_token();

        p
    }

    // cur -> peek, peek -> next
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    // 解析程序
    pub fn parse_program(&mut self) -> Option<Program> {
        // 解析句子 -> 分句子
        let mut program = Program {
            statements: Vec::new(),
        };

        // 分析是不是eof
        while self.cur_token.token_type != TokenType::Eof {
            // 解析句子
            let stmt = self.parse_statement();
            match stmt {
                // 占位
                Statement::None => {}
                _ => {
                    program.statements.push(stmt);
                }
            }
            // 下一个token开始循环
            self.next_token();
        }
        return Some(program);
    }

    // 解析statement
    pub fn parse_statement(&mut self) -> Statement {
        match self.cur_token.token_type {
            // skip error handle later for more
            // TODO: make unwrap() dispear
            TokenType::Let => self.parse_let_statement().unwrap(),
            TokenType::Return => self.parse_return_statement().unwrap(),
            _ => Statement::None,
        }
    }

    // 解析let statement 一个Option<Statement> => Statement::Let{name: Ident, value: Expr}
    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        // 因为我提前预判到cur_token 是TokenType::Let
        // 直接就可以peek 是不是ident
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        // 开始创建Let Statement
        let stmt = Statement::Let {
            name: Ident(self.cur_token.literal.clone()),
            // skip value expression
            value: Expr::Default,
        };

        // let x = y;
        // 确保ident 下一个是assign标志
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // TODO: we are skipping the value handle
        // encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(stmt);
    }

    // 解析return statement => Statement::Returnt{ReturnStatement}
    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        // 因为我已经知道tokenType == TokenType::Return 所以没必要获取literal
        let stmt = Statement::Return(ReturnStatement {
            return_value: Expr::Default,
        });

        // 跳到下一个token  (处理value)
        self.next_token();

        // TODO: We are skipping the expression until we encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(stmt);
    }

    // 辅助函数 查看当前tokentype 是否匹配
    pub fn cur_token_is(&self, token_type: TokenType) -> bool {
        return self.cur_token.token_type == token_type;
    }

    // 辅助函数 查看下一个tokentype 是否匹配
    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        return self.peek_token.token_type == token_type;
    }

    // 如果接下来的类型是和参数token_type 匹配 滚动下一个next token 然后返回true
    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        } else {
            self.peek_errors(token_type);
            return false;
        }
    }

    // errors 辅助函数
    pub fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }

    // peek error 函数 怕出现peek error 然后添加信息到errors
    pub fn peek_errors(&mut self, token_type: TokenType) {
        // 先使用debug
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors().push(msg);
    }
}
