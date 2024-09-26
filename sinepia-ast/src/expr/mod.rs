mod binary;
mod block;
mod call;
mod ifelse;
mod lit;
mod loop_expr;
mod ret;
mod unary;
mod whi;

use crate::{
    enclosed::Parenthesized,
    literals::Ident,
    punctuated::Punctuated,
    token::{Break, Comma, Continue},
};
use std::fmt::Display;

pub use binary::*;
pub use block::*;
pub use call::*;
pub use ifelse::*;
pub use lit::*;
pub use loop_expr::*;
pub use ret::*;
use salsa::Update;
pub use unary::*;
pub use whi::*;

#[derive(Update, Clone, Debug)]
pub enum Expr<'db> {
    Binary(ExprBinary<'db>),
    Block(Block<'db>),
    Break(Break),
    Call(ExprCall<'db>),
    If(ExprIf<'db>),
    Lit(Lit<'db>),
    Loop(ExprLoop<'db>),
    Return(ExprReturn<'db>),
    Continue(Continue),
    Tuple(Parenthesized<Punctuated<Expr<'db>, Comma>>),
    Unary(ExprUnary<'db>),
    While(ExprWhile<'db>),
    Ident(Ident<'db>),
}

impl<'db> Display for Expr<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(inner) => write!(f, "Expr({inner})"),
            Expr::Block(inner) => write!(f, "Expr({inner})"),
            Expr::Break(inner) => write!(f, "Expr({inner})"),
            Expr::Call(inner) => write!(f, "Expr({inner})"),
            Expr::If(inner) => write!(f, "Expr({inner})"),
            Expr::Lit(inner) => write!(f, "Expr({inner})"),
            Expr::Loop(inner) => write!(f, "Expr({inner})"),
            Expr::Return(inner) => write!(f, "Expr({inner})"),
            Expr::Continue(inner) => write!(f, "Expr({inner})"),
            Expr::Tuple(inner) => write!(f, "Expr({inner})"),
            Expr::Unary(inner) => write!(f, "Expr({inner})"),
            Expr::While(inner) => write!(f, "Expr({inner})"),
            Expr::Ident(inner) => write!(f, "Expr({inner})"),
        }
    }
}
