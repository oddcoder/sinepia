use super::Expr;
use crate::{enclosed::Parenthesized, punctuated::Punctuated, token::Comma};
use std::fmt::Display;

#[salsa::tracked]
pub struct ExprCall<'db> {
    pub func: Box<Expr<'db>>,
    pub args: Parenthesized<Punctuated<Expr<'db>, Comma>>,
}

impl<'db> Display for ExprCall<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(
                f,
                "ExprCall{{func: {}, args:{}}}",
                self.func(db),
                self.args(db),
            )
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}
