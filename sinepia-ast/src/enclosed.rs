use crate::token::{BraceClose, BraceOpen, ParenClose, ParenOpen};
use salsa::Update;
use std::fmt::Display;

#[derive(Update, Clone, Debug)]
pub struct Enclosed<Open: Update, T: Update, Close: Update> {
    pub open: Open,
    pub inner: T,
    pub close: Close,
}

impl<O, T, C> Display for Enclosed<O, T, C>
where
    O: Update + Display,
    T: Update + Display,
    C: Update + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enclosed({}, {}, {})", self.open, self.inner, self.close)
    }
}

pub type Parenthesized<T> = Enclosed<ParenOpen, T, ParenClose>;
pub type Braced<T> = Enclosed<BraceOpen, T, BraceClose>;
