use super::{PResult, Parsable};
use crate::ParserCtx;
use salsa::Update;
use sinepia_ast::punctuated::Punctuated;
use std::fmt::Display;

impl<'db, T, P> Parsable<'db> for Punctuated<T, P>
where
    T: Display + Update + Parsable<'db>,
    P: Display + Update + Parsable<'db>,
{
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Punctuated::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Punctuated<T, P>> {
        let mut inner = Vec::new();
        let ret;
        loop {
            let Ok(data) = T::parse(ctx) else {
                ret = Ok(Self { inner, last: None });
                break;
            };
            let Ok(punct) = P::parse(ctx) else {
                ret = Ok(Self {
                    inner,
                    last: Some(Box::new(data)),
                });
                break;
            };
            inner.push((data, punct));
        }
        ret
    }
}
