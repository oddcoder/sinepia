use super::Expr;
use crate::token::Return;
use std::fmt::Display;

#[salsa::tracked]
pub struct ExprReturn<'db> {
    pub return_token: Return,
    pub expr: Option<Box<Expr<'db>>>,
}
impl<'db> Display for ExprReturn<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            if let Some(e) = self.expr(db) {
                write!(f, "Return({e})")
            } else {
                write!(f, "Return@{}", self.return_token(db).span)
            }
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}
