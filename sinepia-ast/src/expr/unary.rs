use super::Expr;
use crate::token::{Minus, Not};
use salsa::Update;
use std::fmt::Display;

#[salsa::tracked]
pub struct ExprUnary<'db> {
    pub op: UnOp,
    pub expr: Box<Expr<'db>>,
}

impl<'db> Display for ExprUnary<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(f, "ExprUnary({} {})", self.op(db), self.expr(db))
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

#[derive(Update, PartialEq, Clone, Debug)]
pub enum UnOp {
    Not(Not),
    Neg(Minus),
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            UnOp::Not(_) => "Not",
            UnOp::Neg(_) => "Neg",
        };
        write!(f, "{op}")
    }
}
