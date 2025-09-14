#[derive(Debug, Clone, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    // Let Statement
    Let { name: Ident, value: Expr },
    None,
}

// Ident: string
#[derive(Debug, Clone, Default)]
pub struct Ident(pub String);

// expression
#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    Integer(i64),
    Float(f64),
}
