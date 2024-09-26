mod peakable;
use peakable::Peakable;
use salsa::Database;
use sinepiac_lexer::Tokens;

pub struct ParserCtx<'db> {
    pub db: &'db dyn Database,
    pub tokens: Tokens<'db>,
    // location of next token to be parsed
    pos: usize,
    // true = do not skip comments
    // false = skip comments
    comments: bool,
    // true = do not skip white space
    // false = skip whitespace
    ws: bool,
}

impl<'db> ParserCtx<'db> {
    pub fn new(db: &'db dyn Database, tokens: Tokens<'db>) -> Self {
        Self {
            db,
            tokens,
            pos: 0,
            comments: false,
            ws: false,
        }
    }
    pub fn with_comments(mut self) -> Self {
        self.comments = true;
        self
    }
    pub fn with_whitespace(mut self) -> Self {
        self.ws = true;
        self
    }
    pub fn peakable<'ctx>(&'ctx mut self) -> Peakable<'ctx, 'db> {
        Peakable::new(self)
    }
}
