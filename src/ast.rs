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

// expression
#[derive(Debug, Clone, Default)]
pub enum Expr {
    // for current skip expr
    #[default]
    Default,
    Ident(Ident),
    Integer(i64),
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
}

// just in case
impl Expr {
    pub fn string(&self) -> String {
        match self {
            Expr::Default => "Default".to_string(),
            Expr::Float(x) => x.to_string(),
            Expr::Ident(i) => i.0.clone(),
            Expr::Integer(it) => it.to_string(),
            Expr::Prefix { op, right } => format!("{:?} {}", op, right.string()),
            Expr::Infix { left, op, right } => {
                format!("{} {:?} {}", left.string(), op, right.string())
            }
        }
    }
}
