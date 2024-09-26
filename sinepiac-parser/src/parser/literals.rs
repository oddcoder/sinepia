use super::PResult;
use crate::{Parsable, ParserCtx};
use sinepia_ast::literals::{Ident, LitBool, LitInt};
use sinepiac_diagnostics::Token as DiagToken;
use sinepiac_lexer::Token;

impl<'db> Parsable<'db> for Ident<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Ident::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token = peakable.expect_one_of(&[DiagToken::Ident])?;
        peakable.commit();
        let src = ctx.tokens.file(ctx.db);
        let span = token.span(ctx.db);
        let name = src.at_span(ctx.db, span);
        Ok(Self::new(ctx.db, name.to_owned(), span))
    }
}

impl<'db> Parsable<'db> for LitBool<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "LitBool::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token = peakable.expect_one_of(&[DiagToken::True, DiagToken::False])?;
        peakable.commit();
        let lit = match token.token(ctx.db) {
            Token::True => true,
            Token::False => false,
            _ => unreachable!(), // thanks to expect_one_of method
        };
        let ret = LitBool::new(ctx.db, lit, token.span(ctx.db));
        Ok(ret)
    }
}

impl<'db> Parsable<'db> for LitInt<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "LitInt::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token = peakable.expect_one_of(&[DiagToken::Number])?;
        peakable.commit();
        let src = ctx.tokens.file(ctx.db);
        let span = token.span(ctx.db);
        let lit = src.at_span(ctx.db, span);
        Ok(Self::new(ctx.db, lit.to_owned(), span))
    }
}
