mod exists;
mod forall;
mod hoare;
mod propbin;

pub use exists::*;
pub use forall::*;
pub use hoare::*;
pub use propbin::*;
use salsa::Update;
use std::fmt::Display;

use crate::literals::Ident;

#[derive(Update, Clone, Debug)]
pub enum Prop<'db> {
    Exist(PropExist<'db>),
    Forall(PropForall<'db>),
    Binary(PropBin<'db>),
    Ident(Ident<'db>),
}
impl<'db> Display for Prop<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prop::Exist(inner) => write!(f, "{inner}"),
            Prop::Forall(inner) => write!(f, "{inner}"),
            Prop::Binary(inner) => write!(f, "{inner}"),
            Prop::Ident(inner) => write!(f, "{inner}"),
        }
    }
}
