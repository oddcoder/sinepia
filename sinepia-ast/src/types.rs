use crate::{literals::Ident, AstVisitor, Receiver};
use salsa::{Database, Update};
use std::fmt::Display;

#[derive(Update, Clone, Debug)]
pub enum Type<'db> {
    Ident(Ident<'db>),
}

impl<'db> Display for Type<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            let Type::Ident(i) = self;
            write!(f, "Type({})@{}", i.data(db), i.span(db))
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

impl<'db, DB: Database, Vis: AstVisitor<'db, DB>> Receiver<'db, DB, Vis> for Type<'db> {
    fn accept(&self, vis: &mut Vis, db: &'db DB) {
        let cont = vis.visit_type_pre(db, self);
        if cont.is_continue() {
            let Type::Ident(ty) =self;
            ty.accept(vis, db);
        }
        vis.visit_type_post(db, self);
    }
}