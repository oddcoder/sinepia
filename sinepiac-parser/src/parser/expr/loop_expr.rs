use crate::{PResult, Parsable};
use sinepia_ast::{
    expr::{Block, ExprLoop},
    token::Loop,
};
impl<'db> Parsable<'db> for ExprLoop<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ExprLoop::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let loop_tok = Loop::parse(ctx)?;
        let body = Block::parse(ctx)?;
        Ok(ExprLoop::new(ctx.db, loop_tok, body))
    }
}
