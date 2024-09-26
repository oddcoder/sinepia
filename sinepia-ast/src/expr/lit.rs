use crate::literals::{LitBool, LitInt};
use salsa::Update;
use std::fmt::Display;

#[derive(Debug, Update, Clone)]
pub enum Lit<'db> {
    Int(LitInt<'db>),
    Bool(LitBool<'db>),
}

impl<'db> Display for Lit<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| match self {
            Self::Int(l) => write!(f, "Lit({})@{}", l.data(db), l.span(db)),
            Self::Bool(l) => write!(f, "Lit({})@{}", l.data(db), l.span(db)),
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}
