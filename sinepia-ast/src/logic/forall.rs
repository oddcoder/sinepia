use super::Prop;
use crate::{
    enclosed::Parenthesized,
    functions::FnArg,
    punctuated::Punctuated,
    token::{Comma, Forall},
};
use std::fmt::Display;

#[salsa::tracked]
pub struct PropForall<'db> {
    pub exists_token: Forall,
    pub witness: Parenthesized<Punctuated<FnArg<'db>, Comma>>,
    pub prop: Box<Prop<'db>>,
}

impl<'db> Display for PropForall<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(
                f,
                "PropForall{{witness: {}, prop: {}}}",
                self.witness(db),
                self.prop(db),
            )
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}
