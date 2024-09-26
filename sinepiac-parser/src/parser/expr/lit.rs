use crate::{PResult, Parsable};
use sinepia_ast::{
    expr::Lit,
    literals::{LitBool, LitInt},
};
use sinepiac_diagnostics::Token as DiagToken;
use sinepiac_lexer::Token;

impl<'db> Parsable<'db> for Lit<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Lit::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token =
            peakable.expect_one_of(&[DiagToken::True, DiagToken::False, DiagToken::Number])?;
        peakable.commit();
        let span = token.span(ctx.db);
        match token.token(ctx.db) {
            Token::True => Ok(Self::Bool(LitBool::new(ctx.db, true, span))),
            Token::False => Ok(Self::Bool(LitBool::new(ctx.db, false, span))),
            Token::Number => Ok(Self::Int(LitInt::new(
                ctx.db,
                ctx.tokens.file(ctx.db).at_span(ctx.db, span).to_owned(),
                span,
            ))),
            _ => unreachable!(),
        }
    }
}
