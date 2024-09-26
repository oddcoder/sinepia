use super::{Block, Expr};
use crate::token::While;
use std::fmt::Display;

#[salsa::tracked]
pub struct ExprWhile<'db> {
    pub while_token: While,
    pub cond: Box<Expr<'db>>,
    pub body: Block<'db>,
}

impl<'db> Display for ExprWhile<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(
                f,
                "While{{cond: {}, body: {}}}",
                self.cond(db),
                self.body(db),
            )
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}
