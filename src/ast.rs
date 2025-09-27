use crate::token::TokenType;

// Program struct
#[derive(Debug, Clone, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn string(&self) -> String {
        let mut out = String::new();
        for s in self.statements.iter() {
            out.push_str(&s.string());
        }
        return out;
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    // Let Statement
    Let { name: Ident, value: Expr },
    // Return Statement
    Return(ReturnStatement),
    // Expression Statement
    Expression(ExpressionStatement),
    Block(BlockStatement),
    None,
}

impl Statement {
    pub fn string(&self) -> String {
        match self {
            Statement::Let { name, value } => {
                format!("let {} = {};", name.string(), value.string())
            }
            Statement::Expression(expression) => expression.string(),
            Statement::Return(returnstmt) => returnstmt.string(),
            Statement::Block(block) => block.string(),
            Statement::None => "None".to_string(),
        }
    }
}

// Ident: string
#[derive(Debug, Clone, Default)]
pub struct Ident(pub String);

impl Ident {
    pub fn string(&self) -> String {
        self.0.clone()
    }
}

// Return Statement: Expr
#[derive(Debug, Clone, Default)]
pub struct ReturnStatement {
    pub return_value: Expr,
}

// 为这些类型授予String 方法
impl ReturnStatement {
    pub fn string(&self) -> String {
        return self.return_value.string();
    }
}

// Expression Statement: Expr
#[derive(Debug, Clone, Default)]
pub struct ExpressionStatement {
    pub expression: Expr,
}

impl ExpressionStatement {
    pub fn string(&self) -> String {
        return self.expression.string();
    }
}

// Function literal expression
#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Ident>,
    pub body: Box<Statement>,
}

impl Function {
    pub fn string(&self) -> String {
        let mut params = Vec::new();
        for p in self.parameters.iter() {
            params.push(p.string());
        }

        format!("fn({}) {}", params.join(", "), self.body.string())
    }
}

// expression
#[derive(Debug, Clone, Default)]
pub enum Expr {
    // for current skip expr
    #[default]
    Default,
    // Identifier
    Ident(Ident),
    // Integer type: i64
    Integer(i64),
    // Float type: f64
    Float(f64),
    // Prefix expression [ -1 ]
    Prefix {
        op: TokenType,
        right: Box<Expr>,
    },
    // Infix expression [ 1 + 1 ]
    Infix {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
    // boolean
    Boolean(bool),
    // if expression
    IfExpression {
        condition: Box<Expr>,
        consequence: Box<Statement>,
        alternative: Box<Statement>, // 后续需要修改为Option<BlockStatement>
    },
    // fn expression
    Fn(Function),
}

// just in case
impl Expr {
    pub fn string(&self) -> String {
        match self {
            Expr::Default => "Default".to_string(),
            Expr::Float(x) => x.to_string(),
            Expr::Ident(i) => i.0.clone(),
            Expr::Integer(it) => it.to_string(),
            Expr::Prefix { op, right } => format!("({}{})", op, right.string()),
            Expr::Infix { left, op, right } => {
                format!("({} {} {})", left.string(), op, right.string())
            }
            Expr::Boolean(b) => b.to_string(),
            Expr::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                let mut out = format!("if{} {}", condition.string(), consequence.string());
                // 骚操作
                if let Statement::None = &**alternative {
                    return out;
                }

                // 添加字符
                out.push_str(&alternative.string());
                out
            }
            Expr::Fn(func) => func.string(),
        }
    }
}

// BlockStatement结构体
#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

// 添加string()方法
impl BlockStatement {
    pub fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        return out;
    }
}
