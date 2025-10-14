use crate::{
    ast::{
        BlockStatement, Expr, ExpressionStatement, Function, Ident, Program, ReturnStatement,
        Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Precedence {
    #[default]
    Lowest = 0,
    Equals,      // == !=
    LessGreater, // >= or > or < or <=
    Sum,         // a + b or a - b
    Product,     // a * b or a / b
    Prefix,      // !a -a +a
    Call,        // call(x)
    Highest,
}

// 为每一个token_type 选择合适的Precedence
impl Precedence {
    #[inline]
    pub fn of(token_type: TokenType) -> Precedence {
        use TokenType::*;
        match token_type {
            Eq | NotEq => Precedence::Equals,
            Lt | Gt | Le | Ge => Precedence::LessGreater,
            Plus | Minus => Precedence::Sum,
            Slash | Asterisk => Precedence::Product,
            Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

#[derive(Debug)]
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

        // 跳跃两次 让token建立正确顺序
        p.next_token();
        p.next_token();

        // 返回p
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
            // 默认处理表达式
            _ => self.parse_expression_statement().unwrap(),
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
        let name = Ident(self.cur_token.literal.clone());
        // skip value expression

        // let x = y;
        // 确保ident 下一个是assign标志
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // = 标志
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest);

        // encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(Statement::Let {
            name: name,
            value: value,
        });
    }

    // 解析return statement => Statement::Returnt{ReturnStatement}
    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        // 因为我已经知道tokenType == TokenType::Return 所以没必要获取literal

        // 跳到下一个token  (处理value)
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest);
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(Statement::Return(ReturnStatement {
            return_value: value,
        }));
    }

    // 解析expresion statement => Statement::Expression(ExpressionStatement)
    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        // 预先是Lowest优先级
        let expression = self.parse_expression(Precedence::Lowest);

        // 如果下一个是; 直接跳一个token
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        // return a Statement
        return Some(Statement::Expression(ExpressionStatement {
            expression: expression,
        }));
    }

    // 解析 Expression 的案例 但是目前错误处理是 Expr::Default 做占位
    pub fn parse_expression(&mut self, prec: Precedence) -> Expr {
        let mut left = match self.cur_token.token_type {
            // 处理Expression 中的 Ident
            TokenType::Ident => Expr::Ident(Ident(self.cur_token.literal.clone())),
            // 处理 Expression 中的 Integer
            // 直接逻辑就是 和monkey go不太一样的事情是 我直接parser为Integer
            TokenType::Int => match self.cur_token.literal.parse::<i64>() {
                Ok(i) => Expr::Integer(i),
                Err(e) => {
                    eprintln!("error parse to integer, {}, set Integer to 0", e);
                    Expr::Integer(0)
                }
            },
            // 解析Prefix式子用的 ! 和 - 和 +
            TokenType::Bang | TokenType::Minus | TokenType::Plus => {
                let op = self.cur_token.token_type.clone();
                self.next_token();
                let right = self.parse_expression(Precedence::Prefix);
                Expr::Prefix {
                    op: op,
                    right: Box::new(right),
                }
            }
            // 处理括号表达式
            TokenType::Lparen => {
                self.next_token();
                let expr = self.parse_expression(Precedence::Lowest);
                if !self.expect_peek(TokenType::Rparen) {
                    return Expr::Default;
                }
                expr
            }
            // 处理boolean
            TokenType::True | TokenType::False => return self.parse_boolean(),
            // 处理if表达式
            TokenType::If => self.parse_if_expression(),
            // 处理Function 函数
            TokenType::Function => self.parse_function(),
            // 默认处理 占位
            _ => Expr::Default,
        };

        // 基于优先级的infix折叠循环
        while !self.peek_token_is(TokenType::Semicolon)
            && self.peek_token.token_type != TokenType::Eof
            && prec < self.peek_precedence()
        {
            let is_infix_or_call = matches!(
                self.peek_token.token_type,
                TokenType::Plus
                    | TokenType::Minus
                    | TokenType::Asterisk
                    | TokenType::Slash
                    | TokenType::Lt
                    | TokenType::Gt
                    | TokenType::Le
                    | TokenType::Ge
                    | TokenType::Eq
                    | TokenType::NotEq
                    | TokenType::Lparen
            );
            // 如果下一个tokentype 不是运算符 operator 那就直接break循环
            if !is_infix_or_call {
                break;
            }
            // 跳转到运算符号
            self.next_token();
            // 然后找到 parser_infix_expression
            // 逻辑:
            //  获取当前运算符的优先级
            //  获取operator
            //  然后跳转到变量或者Expr
            //  解析运算符号
            if self.cur_token_is(TokenType::Lparen) {
                left = self.parse_call_expression(left);
            } else {
                left = self.parse_infix_expression(left);
            }
        }

        // 返回解析好的infix expression
        return left;
    }

    // parse call expression
    pub fn parse_call_expression(&mut self, func: Expr) -> Expr {
        let arguements = match self.parse_call_arguments() {
            Some(args) => args,
            None => panic!("error parsing arguements()"),
        };

        Expr::Call {
            function: Box::new(func),
            arguments: arguements,
        }
    }

    // helper function: parser call expression for arguements
    pub fn parse_call_arguments(&mut self) -> Option<Vec<Expr>> {
        // 初始化args队列
        let mut args = Vec::new();

        // 如果下一个是Rparen 直接退出
        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return Some(args);
        }

        // 如果不是就跳转到第一个参数
        self.next_token();
        // 解析参数表达式
        args.push(self.parse_expression(Precedence::Lowest));

        // 如果下一个是, 说明是多参数 一直到下一个参数不是逗号
        while self.peek_token_is(TokenType::Comma) {
            // a, b 从a 跳跃到comma 然后跳跃到b 跳跃两次
            self.next_token();
            self.next_token();
            // 然后解析参数
            args.push(self.parse_expression(Precedence::Lowest));
        }

        // 解析完 发现没有右括号
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        // 返回args
        Some(args)
    }

    // parse infix
    pub fn parse_infix_expression(&mut self, left: Expr) -> Expr {
        // 提取优先级
        let precedence = self.cur_precedence();
        // 获取操作符号
        let operator = self.cur_token.token_type;
        // 跳转下一个token
        self.next_token();
        // 右侧符号位置 将优先级带入
        let right = self.parse_expression(precedence);
        // 获得infix expression
        let expression = Expr::Infix {
            left: Box::new(left),
            op: operator,
            right: Box::new(right),
        };

        expression
    }

    // parse boolean
    pub fn parse_boolean(&mut self) -> Expr {
        // 返回boolean expression
        Expr::Boolean(self.cur_token_is(TokenType::True))
    }

    // parse if expression
    pub fn parse_if_expression(&mut self) -> Expr {
        // cur_token.TokenType == If
        if !self.expect_peek(TokenType::Lparen) {
            panic!("need (");
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest);

        // 别忘了 expect_peek() 会自己滚动一个token
        if !self.expect_peek(TokenType::Rparen) {
            panic!("need )")
        }

        // 跳转到了{ lbrace
        if !self.expect_peek(TokenType::Lbrace) {
            panic!("need {{");
        }

        // 解析block
        let consequence = self.parse_block_statement();
        let alternative = if self.peek_token_is(TokenType::Else) {
            self.next_token();
            if !self.expect_peek(TokenType::Lbrace) {
                panic!("need {{ after else");
            }
            self.parse_block_statement()
        } else {
            Statement::None
        };

        let expression = Expr::IfExpression {
            condition: Box::new(condition),
            consequence: Box::new(consequence), // statement::Block(BlockStatements)
            alternative: Box::new(alternative),
        };
        return expression;
    }

    // parse fn expression
    pub fn parse_function(&mut self) -> Expr {
        // 先跳转到左括号
        if !self.expect_peek(TokenType::Lparen) {
            panic!("expected (");
        }

        // 然后开始解析 函数参数
        let parameters = self.parse_function_parameters();
        // 跳转到{
        if !self.expect_peek(TokenType::Lbrace) {
            panic!("expected {{ after function parameters");
        }
        // 解析函数block
        let body = self.parse_block_statement();

        // 返回解析好的Expr::Fn(func)
        let func = Expr::Fn(Function {
            parameters: parameters,
            body: Box::new(body),
        });
        return func;
    }

    // parse fn parameters (helper function)
    pub fn parse_function_parameters(&mut self) -> Vec<Ident> {
        // 初始化变量Idents 变量保存
        let mut idents: Vec<Ident> = Vec::new();

        // () 参数为0的情况
        if self.peek_token_is(TokenType::Rparen) {
            // 直接跳转 )
            self.next_token();
            // 返回空参数
            return idents;
        }

        // 如果 不是参数为0 跳转到第一个参数 差不多x, y 的x位置
        self.next_token();
        // 计入x 变量
        idents.push(Ident(self.cur_token.literal.clone()));

        // 如果下一个是, 那么跳转
        while self.peek_token_is(TokenType::Comma) {
            // x_(当前在x), y, 距离下一个变量总是相差2个身位
            self.next_token();
            self.next_token();
            // 跳转到了 y
            idents.push(Ident(self.cur_token.literal.clone()));
        }

        // 如果下一个不是) 直接panic 如果是 跳转到了 )
        if !self.expect_peek(TokenType::Rparen) {
            panic!("expected )")
        }

        return idents;
    }

    // parse block statement
    pub fn parse_block_statement(&mut self) -> Statement {
        // 初始化语句解析
        let mut statements: Vec<Statement> = Vec::new();

        // 之前有self.next_token() 目前在{ 跳转到了block内部
        self.next_token();

        // 如果{} 为空 那么直接退出
        while !self.cur_token_is(TokenType::Rbrace) && !self.cur_token_is(TokenType::Eof) {
            let stmt = self.parse_statement();
            match stmt {
                Statement::None => {
                    self.errors
                        .push("failed to parse statement inside block".into());
                }
                _ => statements.push(stmt),
            }

            self.next_token();
        }

        return Statement::Block(BlockStatement {
            statements: statements,
        });
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

    // 辅助查看cur_token的优先级
    pub fn cur_precedence(&self) -> Precedence {
        Precedence::of(self.cur_token.token_type)
    }

    // 辅助查看peek_token的优先级
    pub fn peek_precedence(&self) -> Precedence {
        Precedence::of(self.peek_token.token_type)
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
        self.errors.push(msg);
    }
}
