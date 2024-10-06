use crate::Span;
use salsa::Database;
use std::path::PathBuf;

#[salsa::input]
pub struct SourceFile {
    #[return_ref]
    pub path: PathBuf,
    #[return_ref]
    pub content: String,
}

impl SourceFile {
    pub fn last_span(&self, db: &dyn Database) -> Span {
        let content: &String = self.content(db);
        let len = content.len() as u32;
        Span::default().with_lo(len - 1).with_hi(len - 1)
    }
    pub fn at_span<'a>(&'a self, db: &'a dyn Database, span: Span) -> &'a str {
        let content: &String = self.content(db);
        content.get(span.lo as usize..span.hi as usize).unwrap()
    }
}
