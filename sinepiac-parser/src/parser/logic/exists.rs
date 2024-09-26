use crate::{PResult, Parsable, ParserCtx};
use sinepia_ast::{
    enclosed::Parenthesized,
    logic::{Prop, PropExist},
    token::Exists,
};

impl<'db> Parsable<'db> for PropExist<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "PropExist::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let tok = Exists::parse(ctx)?;
        let witnss = Parenthesized::parse(ctx)?;
        let prop = Prop::parse(ctx)?;
        Ok(Self::new(ctx.db, tok, witnss, Box::new(prop)))
    }
}
