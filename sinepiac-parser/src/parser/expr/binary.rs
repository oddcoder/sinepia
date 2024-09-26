use crate::{PResult, Parsable, ParserCtx};
use sinepia_ast::{
    expr::{BinOp, Expr, ExprBinary},
    token::{
        And, AndAnd, AndEq, Caret, CaretEq, Eq, EqEq, Ge, Gt, Le, Lt, Minus, MinusEq, Ne, Or, OrEq,
        OrOr, Percent, PercentEq, Plus, PlusEq, Shl, ShlEq, Shr, ShrEq, Slash, SlashEq, Star,
        StarEq,
    },
};
use sinepiac_diagnostics::Token as DiagToken;
use sinepiac_lexer::Token;

impl<'db> Parsable<'db> for ExprBinary<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ExprBinary::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let left = Expr::parse(ctx)?;
        let op = BinOp::parse(ctx)?;
        let right = Expr::parse(ctx)?;
        Ok(Self::new(ctx.db, Box::new(left), op, Box::new(right)))
    }
}

const EXPECTED_OP: &[DiagToken] = &[
    DiagToken::Plus,
    DiagToken::Minus,
    DiagToken::Star,
    DiagToken::Slash,
    DiagToken::Percent,
    DiagToken::AndAnd,
    DiagToken::OrOr,
    DiagToken::Caret,
    DiagToken::And,
    DiagToken::Or,
    DiagToken::Shl,
    DiagToken::Shr,
    DiagToken::EqEq,
    DiagToken::Eq,
    DiagToken::Lt,
    DiagToken::Le,
    DiagToken::Ne,
    DiagToken::Ge,
    DiagToken::Gt,
    DiagToken::PlusEq,
    DiagToken::MinusEq,
    DiagToken::StarEq,
    DiagToken::SlashEq,
    DiagToken::PercentEq,
    DiagToken::CaretEq,
    DiagToken::AndEq,
    DiagToken::OrEq,
    DiagToken::ShlEq,
    DiagToken::ShrEq,
];
impl<'db> Parsable<'db> for BinOp {
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token = peakable.expect_one_of(EXPECTED_OP)?;
        peakable.commit();
        match token.token(ctx.db) {
            Token::Plus => Ok(Self::Add(Plus {
                span: token.span(ctx.db),
            })),
            Token::Minus => Ok(Self::Sub(Minus {
                span: token.span(ctx.db),
            })),
            Token::Star => Ok(Self::Mul(Star {
                span: token.span(ctx.db),
            })),
            Token::Slash => Ok(Self::Div(Slash {
                span: token.span(ctx.db),
            })),
            Token::Percent => Ok(Self::Rem(Percent {
                span: token.span(ctx.db),
            })),
            Token::AndAnd => Ok(Self::And(AndAnd {
                span: token.span(ctx.db),
            })),
            Token::OrOr => Ok(Self::Or(OrOr {
                span: token.span(ctx.db),
            })),
            Token::Caret => Ok(Self::BitXor(Caret {
                span: token.span(ctx.db),
            })),
            Token::And => Ok(Self::BitAnd(And {
                span: token.span(ctx.db),
            })),
            Token::Or => Ok(Self::BitOr(Or {
                span: token.span(ctx.db),
            })),
            Token::Shl => Ok(Self::Shl(Shl {
                span: token.span(ctx.db),
            })),
            Token::Shr => Ok(Self::Shr(Shr {
                span: token.span(ctx.db),
            })),
            Token::EqEq => Ok(Self::EqEq(EqEq {
                span: token.span(ctx.db),
            })),
            Token::Eq => Ok(Self::Eq(Eq {
                span: token.span(ctx.db),
            })),
            Token::Lt => Ok(Self::Lt(Lt {
                span: token.span(ctx.db),
            })),
            Token::Le => Ok(Self::Le(Le {
                span: token.span(ctx.db),
            })),
            Token::Ne => Ok(Self::Ne(Ne {
                span: token.span(ctx.db),
            })),
            Token::Ge => Ok(Self::Ge(Ge {
                span: token.span(ctx.db),
            })),
            Token::Gt => Ok(Self::Gt(Gt {
                span: token.span(ctx.db),
            })),
            Token::PlusEq => Ok(Self::AddAssign(PlusEq {
                span: token.span(ctx.db),
            })),
            Token::MinusEq => Ok(Self::SubAssign(MinusEq {
                span: token.span(ctx.db),
            })),
            Token::StarEq => Ok(Self::MulAssign(StarEq {
                span: token.span(ctx.db),
            })),
            Token::SlashEq => Ok(Self::DivAssign(SlashEq {
                span: token.span(ctx.db),
            })),
            Token::PercentEq => Ok(Self::RemAssign(PercentEq {
                span: token.span(ctx.db),
            })),
            Token::CaretEq => Ok(Self::BitXorAssign(CaretEq {
                span: token.span(ctx.db),
            })),
            Token::AndEq => Ok(Self::BitAndAssign(AndEq {
                span: token.span(ctx.db),
            })),
            Token::OrEq => Ok(Self::BitOrAssign(OrEq {
                span: token.span(ctx.db),
            })),
            Token::ShlEq => Ok(Self::ShlAssign(ShlEq {
                span: token.span(ctx.db),
            })),
            Token::ShrEq => Ok(Self::ShrAssign(ShrEq {
                span: token.span(ctx.db),
            })),
            _ => unreachable!(),
        }
    }
}
