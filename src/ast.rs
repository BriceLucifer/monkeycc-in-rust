#[derive(Debug, Clone, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    // Let Statement
    Let { name: Ident, value: Expr },
    // Return Expression
    Return(Expr),
    None,
}

// Ident: string
#[derive(Debug, Clone, Default)]
pub struct Ident(pub String);

// Return Statement: Expr
#[derive(Debug, Clone, Default)]
pub struct ReturnStatement {
    pub return_value: Expr,
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
}

// just in case
// impl Expr {
//     pub fn token_literal(&self) -> String {
//         match self {
//             Expr::Default => "Default".to_string(),
//             Expr::Float(x) => x.to_string(),
//             Expr::Ident(i) => i.0.clone(),
//             Expr::Integer(it) => it.to_string(),
//         }
//     }
// }
