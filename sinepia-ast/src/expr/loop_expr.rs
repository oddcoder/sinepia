use super::Block;
use crate::token::Loop;
use std::fmt::Display;

#[salsa::tracked]
pub struct ExprLoop<'db> {
    pub loop_token: Loop,
    pub body: Block<'db>,
}

impl<'db> Display for ExprLoop<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| write!(f, "Loop({})", self.body(db),))
            .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}
