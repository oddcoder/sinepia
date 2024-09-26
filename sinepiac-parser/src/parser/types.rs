use super::Parsable;
use crate::ParserCtx;
use sinepia_ast::{literals::Ident, types::Type};

impl<'db> Parsable<'db> for Type<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Type::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> super::PResult<Self> {
        let ident = Ident::parse(ctx)?;
        Ok(Self::Ident(ident))
    }
}
