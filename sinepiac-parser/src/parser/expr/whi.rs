use crate::Parsable;
use sinepia_ast::{
    expr::{Block, Expr, ExprWhile},
    token::While,
};

impl<'db> Parsable<'db> for ExprWhile<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ExprWhile::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> crate::PResult<Self> {
        let whi = While::parse(ctx)?;
        let cond = Box::new(Expr::parse(ctx)?);
        let body = Block::parse(ctx)?;
        Ok(ExprWhile::new(ctx.db, whi, cond, body))
    }
}
