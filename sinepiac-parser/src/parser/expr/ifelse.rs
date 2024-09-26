use crate::{PResult, Parsable};
use sinepia_ast::{
    expr::{Block, Expr, ExprIf},
    token::{Else, If},
};

impl<'db> Parsable<'db> for ExprIf<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ExprIf::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let if_tok = If::parse(ctx)?;
        let cond = Box::new(Expr::parse(ctx)?);
        let then_block = Block::parse(ctx)?;
        if let Ok(else_tok) = Else::parse(ctx) {
            let else_block = Block::parse(ctx)?;
            Ok(ExprIf::new(
                ctx.db,
                if_tok,
                cond,
                then_block,
                Some((else_tok, else_block)),
            ))
        } else {
            Ok(ExprIf::new(ctx.db, if_tok, cond, then_block, None))
        }
    }
}
