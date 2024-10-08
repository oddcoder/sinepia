use super::{expr::Block, token::Fn, types::Type};
use crate::{
    enclosed::Parenthesized,
    literals::Ident,
    punctuated::Punctuated,
    token::{Colon, Comma, RArrow},
    AstVisitor, Receiver, VisitingStatus,
};
use salsa::{Database, Update};
use std::fmt::Display;

#[salsa::tracked]
pub struct Signature<'db> {
    pub fn_token: Fn,
    pub ident: Ident<'db>,
    pub inputs: Parenthesized<Punctuated<FnArg<'db>, Comma>>,
    pub output: ReturnType<'db>,
}
impl<'db> Display for Signature<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(
                f,
                "Signature{{ ident: {}, inputs: {}, output: {}}}",
                self.ident(db),
                self.inputs(db),
                self.output(db)
            )
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

impl<'db, DB: Database, Vis: AstVisitor<'db, DB>> Receiver<'db, DB, Vis> for Signature<'db> {
    fn accept(&self, vis: &mut Vis, db: &'db DB) {
        let cont = vis.visit_sig_pre(db, *self);
        if cont.is_continue() {
            self.ident(db).accept(vis, db);
            for input in self.inputs(db).inner.iter() {
                input.accept(vis, db)
            }
            self.output(db).accept(vis, db);
        }
        vis.visit_sig_post(db, *self);
    }
}

#[salsa::tracked]
pub struct ItemFn<'db> {
    pub sig: Signature<'db>,
    pub block: Box<Block<'db>>,
}

impl<'db> ItemFn<'db> {
    pub fn name(&self, db: &'db impl Database) -> &str {
        self.sig(db).ident(db).data(db)
    }
}

impl<'db> Display for ItemFn<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(
                f,
                "ItemFn{{sig: {}, block: {}}}",
                self.sig(db),
                self.block(db),
            )
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

impl<'db, DB: Database, Vis: AstVisitor<'db, DB>> Receiver<'db, DB, Vis> for ItemFn<'db> {
    fn accept(&self, vis: &mut Vis, db: &'db DB) {
        let cont = vis.visit_fn_pre(db, *self);
        if let VisitingStatus::Continue = cont {
            let sig_ret = self.sig(db).accept(vis, db);
            let block_ret = self.block(db).accept(vis, db);
            Some((sig_ret, block_ret))
        } else {
            None
        };
        vis.visit_fn_post(db, *self)
    }
}

#[derive(Update, Clone, Debug)]
pub enum ReturnType<'db> {
    Default,
    Type(RArrow, Type<'db>),
}

impl<'db, DB: Database, Vis: AstVisitor<'db, DB>> Receiver<'db, DB, Vis> for ReturnType<'db> {
    fn accept(&self, vis: &mut Vis, db: &'db DB) {
        let cont = vis.visit_return_type_pre(db, self);
        if cont.is_continue() {
            if let ReturnType::Type(_, ty) = self {
                ty.accept(vis, db);
            }
        }
        vis.visit_return_type_post(db, self);
    }
}

impl<'db> Display for ReturnType<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnType::Default => write!(f, "ReturnType(Unit)"),
            ReturnType::Type(_, ty) => write!(f, "ReturnType({ty})"),
        }
    }
}

#[salsa::tracked]
pub struct FnArg<'db> {
    pub name: Ident<'db>,
    pub colon_token: Colon,
    pub ty: Type<'db>,
}

impl<'db> Display for FnArg<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| write!(f, "FnArg({}: {})", self.name(db), self.ty(db)))
            .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

impl<'db, DB: Database, Vis: AstVisitor<'db, DB>> Receiver<'db, DB, Vis> for FnArg<'db> {
    fn accept(&self, vis: &mut Vis, db: &'db DB) {
        todo!()
    }
}
