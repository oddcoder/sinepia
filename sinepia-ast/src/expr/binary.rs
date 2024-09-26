use super::Expr;
use crate::token::{
    And, AndAnd, AndEq, Caret, CaretEq, Eq, EqEq, Ge, Gt, Le, Lt, Minus, MinusEq, Ne, Or, OrEq,
    OrOr, Percent, PercentEq, Plus, PlusEq, Shl, ShlEq, Shr, ShrEq, Slash, SlashEq, Star, StarEq,
};
use salsa::Update;
use std::fmt::Display;

#[salsa::tracked]
pub struct ExprBinary<'db> {
    pub left: Box<Expr<'db>>,
    pub op: BinOp,
    pub right: Box<Expr<'db>>,
}
impl<'db> Display for ExprBinary<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(
                f,
                "ExprBinary{{left: {}, op: {}, right: {}}}",
                self.left(db),
                self.op(db),
                self.right(db)
            )
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

#[derive(Update, Clone, Debug, Copy)]
pub enum BinOp {
    Add(Plus),
    Sub(Minus),
    Mul(Star),
    Div(Slash),
    Rem(Percent),
    And(AndAnd),
    Or(OrOr),
    BitXor(Caret),
    BitAnd(And),
    BitOr(Or),
    Shl(Shl),
    Shr(Shr),
    EqEq(EqEq),
    Lt(Lt),
    Le(Le),
    Ne(Ne),
    Ge(Ge),
    Gt(Gt),
    AddAssign(PlusEq),
    SubAssign(MinusEq),
    MulAssign(StarEq),
    DivAssign(SlashEq),
    RemAssign(PercentEq),
    BitXorAssign(CaretEq),
    BitAndAssign(AndEq),
    BitOrAssign(OrEq),
    ShlAssign(ShlEq),
    ShrAssign(ShrEq),
    Eq(Eq),
}
impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            BinOp::Add(_) => "Add",
            BinOp::Sub(_) => "Sub",
            BinOp::Mul(_) => "Mul",
            BinOp::Div(_) => "Div",
            BinOp::Rem(_) => "Rem",
            BinOp::And(_) => "And",
            BinOp::Or(_) => "Or",
            BinOp::BitXor(_) => "BitXor",
            BinOp::BitAnd(_) => "BitAnd",
            BinOp::BitOr(_) => "BitOr",
            BinOp::Shl(_) => "Shl",
            BinOp::Shr(_) => "Shr",
            BinOp::EqEq(_) => "EqEq",
            BinOp::Lt(_) => "Lt",
            BinOp::Le(_) => "Le",
            BinOp::Ne(_) => "Ne",
            BinOp::Ge(_) => "Ge",
            BinOp::Gt(_) => "Gt",
            BinOp::AddAssign(_) => "AddAssign",
            BinOp::SubAssign(_) => "SubAssign",
            BinOp::MulAssign(_) => "MulAssign",
            BinOp::DivAssign(_) => "DivAssign",
            BinOp::RemAssign(_) => "RemAssign",
            BinOp::BitXorAssign(_) => "BitXorAssign",
            BinOp::BitAndAssign(_) => "BitAndAssign",
            BinOp::BitOrAssign(_) => "BitOrAssign",
            BinOp::ShlAssign(_) => "ShlAssign",
            BinOp::ShrAssign(_) => "ShrAssign",
            BinOp::Eq(_) => "Eq",
        };
        write!(f, "{x}")
    }
}
