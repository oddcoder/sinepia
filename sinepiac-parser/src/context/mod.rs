mod peakable;
use peakable::Peakable;
use salsa::Database;
use sinepiac_lexer::Tokens;

pub struct ParserCtx<'db> {
    pub db: &'db dyn Database,
    pub tokens: Tokens<'db>,
    // location of next token to be parsed
    pos: usize,
}

impl<'db> ParserCtx<'db> {
    pub fn new(db: &'db dyn Database, tokens: Tokens<'db>) -> Self {
        Self {
            db,
            tokens,
            pos: 0,
        }
    }
    pub fn peakable<'ctx>(&'ctx mut self) -> Peakable<'ctx, 'db> {
        Peakable::new(self)
    }
}
