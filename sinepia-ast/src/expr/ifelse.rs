use super::{Block, Expr};
use crate::token::{Else, If};
use std::fmt::Display;

#[salsa::tracked]
pub struct ExprIf<'db> {
    pub if_token: If,
    pub cond: Box<Expr<'db>>,
    pub then_branch: Block<'db>,
    pub else_branch: Option<(Else, Block<'db>)>,
}

impl<'db> Display for ExprIf<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(
                f,
                "ExprIf{{cond: {}, then_branch:{}",
                self.cond(db),
                self.then_branch(db),
            )?;
            if let Some(else_branch) = self.else_branch(db) {
                write!(f, ", else_branch:{} }}", else_branch.1)
            } else {
                write!(f, "}}")
            }
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}
