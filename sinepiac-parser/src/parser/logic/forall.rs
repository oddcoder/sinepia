use crate::{PResult, Parsable, ParserCtx};
use sinepia_ast::{
    enclosed::Parenthesized,
    logic::{Prop, PropForall},
    token::Forall,
};

impl<'db> Parsable<'db> for PropForall<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "PropForall::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let tok = Forall::parse(ctx)?;
        let witnss = Parenthesized::parse(ctx)?;
        let prop = Prop::parse(ctx)?;
        Ok(Self::new(ctx.db, tok, witnss, Box::new(prop)))
    }
}
