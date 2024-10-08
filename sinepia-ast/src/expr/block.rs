use super::Expr;
use crate::{
    enclosed::Braced,
    literals::Ident,
    token::{Eq, Let, Semi},
    AstVisitor, Receiver,
};
use salsa::{Database, Update};
use std::fmt::Display;

#[salsa::tracked]
pub struct Block<'db> {
    pub stmts: Braced<Vec<Stmt<'db>>>,
}

impl<'db> Display for Block<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(f, "Block(")?;
            let list: Vec<_> = self
                .stmts(db)
                .inner
                .iter()
                .map(|stmt| format!("{stmt}"))
                .collect();
            write!(f, "{})", list.join("; "))
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

impl<'db, DB: Database, Vis: AstVisitor<'db, DB>> Receiver<'db, DB, Vis> for Block<'db> {
    fn accept(&self, vis: &mut Vis, db: &'db DB) {
        todo!()
    }
}

#[salsa::tracked]
pub struct Local<'db> {
    pub let_token: Let,
    pub ident: Ident<'db>,
    pub eq_token: Eq,
    pub expr: Expr<'db>,
    pub semi_token: Semi,
}

impl<'db> Display for Local<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(
                f,
                "Local{{ident: {}, expr: {}}}",
                self.ident(db),
                self.expr(db)
            )
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

#[derive(Update, Clone, Debug)]
pub enum Stmt<'db> {
    Local(Local<'db>),
    Expr(Expr<'db>, Option<Semi>),
}

impl<'db> Display for Stmt<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Local(local) => write!(f, "Stmt({local})"),
            Stmt::Expr(expr, _) => write!(f, "Stmt({expr})"),
        }
    }
}
