use crate::literals::Ident;
use salsa::Update;
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
