use super::{PResult, Parsable};
use sinepia_ast::{
    enclosed::Parenthesized,
    expr::Block,
    functions::{FnArg, ItemFn, ReturnType, Signature},
    literals::Ident,
    token::{Colon, Fn, RArrow},
    types::Type,
};

impl<'db> Parsable<'db> for Signature<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Signature::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let fn_token = Fn::parse(ctx)?;
        let ident = Ident::parse(ctx)?;
        let inputs = Parenthesized::parse(ctx)?;
        let output = ReturnType::parse(ctx)?;
        Ok(Self::new(ctx.db, fn_token, ident, inputs, output))
    }
}

impl<'db> Parsable<'db> for ItemFn<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ItemFn::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let sig = Signature::parse(ctx)?;
        let block = Block::parse(ctx)?;
        Ok(Self::new(ctx.db, sig, Box::new(block)))
    }
}

impl<'db> Parsable<'db> for ReturnType<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ReturnType::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let Ok(rarrow) = RArrow::parse(ctx) else {
            return Ok(ReturnType::Default);
        };
        let ty = Type::parse(ctx)?;
        Ok(ReturnType::Type(rarrow, ty))
    }
}

impl<'db> Parsable<'db> for FnArg<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "FnArg::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let name = Ident::parse(ctx)?;
        let colon = Colon::parse(ctx)?;
        let ty = Type::parse(ctx)?;
        Ok(Self::new(ctx.db, name, colon, ty))
    }
}
