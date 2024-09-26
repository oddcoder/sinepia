use crate::{PResult, Parsable};
use sinepia_ast::{
    enclosed::Parenthesized,
    expr::{Expr, ExprCall},
};

impl<'db> Parsable<'db> for ExprCall<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ExprCall::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let func = Box::new(Expr::parse(ctx)?);
        let args = Parenthesized::parse(ctx)?;
        Ok(ExprCall::new(ctx.db, func, args))
    }
}
