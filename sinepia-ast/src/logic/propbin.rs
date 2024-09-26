use super::Prop;
use crate::token::{Conjunction, Disjunction, Implication, MagicWand, StarStar};
use salsa::Update;
use std::fmt::Display;

#[salsa::tracked]
pub struct PropBin<'db> {
    pub left: Box<Prop<'db>>,
    pub op: BinOp,
    pub right: Box<Prop<'db>>,
}

impl<'db> Display for PropBin<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            write!(
                f,
                "PropBin{{left: {}, op: {}, right: {}}}",
                self.left(db),
                self.op(db),
                self.right(db),
            )
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

#[derive(Update, Clone, Debug, Copy)]
pub enum BinOp {
    Conjunction(Conjunction),
    Disjunction(Disjunction),
    Implication(Implication),
    AndSeparately(StarStar),
    MagicWand(MagicWand),
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Conjunction(_) => write!(f, "Conjunction"),
            BinOp::Disjunction(_) => write!(f, "Disjunction"),
            BinOp::Implication(_) => write!(f, "Implication"),
            BinOp::AndSeparately(_) => write!(f, "AndSeparately"),
            BinOp::MagicWand(_) => write!(f, "MagicWand"),
        }
    }
}
