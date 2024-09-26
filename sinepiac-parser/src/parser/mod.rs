mod enclosed;
mod expr;
mod function;
mod literals;
mod logic;
mod module;
mod punctuated;
mod token;
mod types;

use crate::ParserCtx;
use sinepiac_diagnostics::Diagnostic;

pub type PResult<T> = Result<T, Diagnostic>;

pub trait Parsable<'db>: Sized {
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self>;
}

impl<'db, T> Parsable<'db> for Vec<T>
where
    T: Parsable<'db>,
{
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let mut data = Vec::new();
        loop {
            let Ok(item) = T::parse(ctx) else {
                break;
            };
            data.push(item);
        }
        Ok(data)
    }
}
