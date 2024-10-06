use super::ParserCtx;
use sinepiac_diagnostics::{
    early_eof::EarlyEof, unexpected_token::UnexpectedToken, Diagnostic, Token as DiagToken,
};
use sinepiac_lexer::{SpannedToken, Token};

pub struct Peakable<'ctx, 'db> {
    pub ctx: &'ctx mut ParserCtx<'db>,
    pos: usize,
}

impl<'ctx, 'db> Peakable<'ctx, 'db> {
    pub fn new(ctx: &'ctx mut ParserCtx<'db>) -> Self {
        let pos = ctx.pos;
        Self { ctx, pos }
    }
    pub fn commit(self) {
        self.ctx.pos = self.pos
    }
    fn ignore_tokens(&mut self) {
        let toks = self.ctx.tokens.tokens(self.ctx.db);
        while let Some(st) = toks.get(self.pos) {
            match st.token(self.ctx.db) {
                Token::Space | Token::Comment => {
                    self.pos += 1;
                }
                _ => break,
            }
        }
    }
    pub fn expect_one_of(&mut self, tokens: &[DiagToken]) -> Result<SpannedToken<'db>, Diagnostic> {
        let current = self.pos;
        let Some(token) = self.next() else {
            let src = self.ctx.tokens.file(self.ctx.db);
            let span = src.last_span(self.ctx.db);
            let err = EarlyEof::new(src, tokens, span);
            self.pos = current;
            return Err(err.into());
        };
        let tok = token.token(self.ctx.db).into();
        let span = token.span(self.ctx.db);
        if tokens.contains(&tok) {
            return Ok(token);
        }
        let src = self.ctx.tokens.file(self.ctx.db);
        let err = UnexpectedToken::new(src, tokens, tok, span);
        self.pos = current;
        Err(err.into())
    }
}

impl<'ctx, 'db> Iterator for Peakable<'ctx, 'db> {
    type Item = SpannedToken<'db>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ignore_tokens();
        let toks = self.ctx.tokens.tokens(self.ctx.db);
        if let Some(st) = toks.get(self.pos) {
            self.pos += 1;
            Some(*st)
        } else {
            None
        }
    }
}
