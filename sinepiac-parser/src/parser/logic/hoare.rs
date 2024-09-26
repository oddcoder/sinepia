use super::{PResult, Parsable};
use crate::ParserCtx;
use salsa::Update;
use sinepia_ast::{
    logic::{HoareTriplet, Prop},
    token::{Assuming, Comma, Ergo, Semi},
};
use std::fmt::Display;

impl<'db, T> Parsable<'db> for HoareTriplet<'db, T>
where
    T: Display + Update + Parsable<'db>,
{
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "HoareTriplet::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let assume_token = Assuming::parse(ctx)?;
        let precondition = Prop::parse(ctx)?;
        let comma_token = Comma::parse(ctx)?;
        let inner = T::parse(ctx)?;
        let ergo_token = Ergo::parse(ctx)?;
        let post_condition = Prop::parse(ctx)?;
        let semi_token = Semi::parse(ctx)?;
        Ok(HoareTriplet {
            assume_token,
            precondition,
            comma_token,
            inner,
            ergo_token,
            post_condition,
            semi_token,
        })
    }
}
