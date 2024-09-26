use salsa::Update;
use sinepiac_span::Span;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct And {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct AndAnd {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct AndEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct StarStar {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Assuming {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct BraceClose {
    pub span: Span,
}

impl Display for BraceClose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BraceClose")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct BraceOpen {
    pub span: Span,
}
impl Display for BraceOpen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BraceOpen")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Break {
    pub span: Span,
}
impl Display for Break {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Break@{}", self.span)
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Caret {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct CaretEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Colon {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Comma {
    pub span: Span,
}

impl Display for Comma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Comma")
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Conjunction {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Continue {
    pub span: Span,
}
impl Display for Continue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Continue@{}", self.span)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Disjunction {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Else {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Eq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct EqEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Ergo {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Exists {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Fn {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Forall {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Ge {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Gt {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct If {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Implication {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Le {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Let {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Lt {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Loop {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct MagicWand {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Minus {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct MinusEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Ne {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Not {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Or {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct OrEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct OrOr {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct ParenClose {
    pub span: Span,
}
impl Display for ParenClose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParenClose")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct ParenOpen {
    pub span: Span,
}
impl Display for ParenOpen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParenOpen")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Percent {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct PercentEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Plus {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct PlusEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Proof {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Qed {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct RArrow {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Return {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Semi {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Shl {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct ShlEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Shr {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct ShrEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Slash {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct SlashEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct Star {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct StarEq {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Update)]
pub struct While {
    pub span: Span,
}
