use sinepiac_span::Span;
use std::fmt::Display;

#[salsa::interned]
pub struct Ident<'db> {
    #[return_ref]
    pub data: String,
    pub span: Span,
}

impl<'db> Display for Ident<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| write!(f, "Ident({})@{}", self.data(db), self.span(db)))
            .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

#[salsa::interned]
pub struct LitBool<'db> {
    pub data: bool,
    pub span: Span,
}
impl<'db> Display for LitBool<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(f, "LitBool({})@{}", self.data(db), self.span(db))
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

#[salsa::interned]
pub struct LitInt<'db> {
    #[return_ref]
    pub data: String,
    pub span: Span,
}
impl<'db> Display for LitInt<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| write!(f, "LitInt({})@{}", self.data(db), self.span(db)))
            .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}
