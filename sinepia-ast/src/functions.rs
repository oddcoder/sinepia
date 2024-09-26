use super::{expr::Block, token::Fn, types::Type};
use crate::{
    enclosed::Parenthesized,
    literals::Ident,
    punctuated::Punctuated,
    token::{Colon, Comma, RArrow},
};
use salsa::Update;
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

#[salsa::tracked]
pub struct ItemFn<'db> {
    pub sig: Signature<'db>,
    pub block: Box<Block<'db>>,
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

#[derive(Update, Clone, Debug)]
pub enum ReturnType<'db> {
    Default,
    Type(RArrow, Type<'db>),
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
