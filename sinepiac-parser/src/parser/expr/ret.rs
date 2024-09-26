use crate::{PResult, Parsable};
use sinepia_ast::{
    expr::{Expr, ExprReturn},
    token::Return,
};

impl<'db> Parsable<'db> for ExprReturn<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ExprRet::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let ret = Return::parse(ctx)?;
        let expr = if let Ok(expr) = Expr::parse(ctx) {
            Some(Box::new(expr))
        } else {
            None
        };
        Ok(Self::new(ctx.db, ret, expr))
    }
}
