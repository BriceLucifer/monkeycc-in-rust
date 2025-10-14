use crate::{
    ast::{Expr, Program, Statement},
    object::Object,
};

pub fn eval(node: Program) -> Object {
    // try the first statement
    // the first node is statement
    let mut result = Object::None;
    for stmt in &node.statements {
        match stmt {
            Statement::Expression(expr) => match expr.expression {
                Expr::Integer(i) => result = Object::Integer(i),
                Expr::Boolean(b) => result = Object::Boolean(b),
                _ => result = Object::None,
            },
            _ => result = Object::None,
        }
    }
    result
}
