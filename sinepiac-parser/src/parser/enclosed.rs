use super::PResult;
use crate::{Parsable, ParserCtx};
use salsa::Update;
use sinepia_ast::enclosed::Enclosed;

impl<'db, O, T, C> Parsable<'db> for Enclosed<O, T, C>
where
    O: Update + Parsable<'db>,
    T: Update + Parsable<'db>,
    C: Update + Parsable<'db>,
{
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let open = O::parse(ctx)?;
        let inner = T::parse(ctx)?;
        let close = C::parse(ctx)?;
        Ok(Self { open, inner, close })
    }
}
