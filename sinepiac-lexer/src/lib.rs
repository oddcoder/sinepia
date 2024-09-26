#[cfg(test)]
mod test;
mod tokens;

use logos::Logos;
use salsa::{Accumulator, Database};
use sinepiac_diagnostics::{unknown_token::UnknownToken, Diagnostic};
use sinepiac_span::{SourceFile, Span};
pub use tokens::*;

fn tokenize_inner<'db>(db: &'db dyn Database, src: &str) -> (Vec<SpannedToken<'db>>, Vec<Span>) {
    let mut toks = Vec::new();
    let mut errors = Vec::new();
    let mut lex = Token::lexer(src);
    while let Some(tok) = lex.next() {
        let span = lex.span();
        let span = Span::default()
            .with_lo(span.start.try_into().unwrap())
            .with_hi(span.end.try_into().unwrap());
        let Ok(token) = tok else {
            errors.push(span);
            continue;
        };
        toks.push(SpannedToken::new(db, token, span));
    }
    (toks, errors)
}

#[salsa::tracked]
pub fn lex_file<'db>(db: &'db dyn Database, src: SourceFile) -> Tokens<'db> {
    let (tokens, errors) = tokenize_inner(db, src.content(db));
    for span in errors {
        let diag: Diagnostic = UnknownToken::new(src, span).into();
        diag.accumulate(db);
    }
    Tokens::new(db, src, tokens)
}
