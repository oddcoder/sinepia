use super::{PResult, Parsable};
use crate::ParserCtx;
use sinepia_ast::{
    logic::{BinOp, Prop, PropBin},
    token::{Conjunction, Disjunction, Implication, MagicWand, StarStar},
};
use sinepiac_diagnostics::Token as DiagToken;
use sinepiac_lexer::Token;

impl<'db> Parsable<'db> for PropBin<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "PropBin::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let lhs = Box::new(Prop::parse(ctx)?);
        let op = BinOp::parse(ctx)?;
        let rhs = Box::new(Prop::parse(ctx)?);
        Ok(Self::new(ctx.db, lhs, op, rhs))
    }
}

const EXPECTED_OP: &[DiagToken] = &[
    DiagToken::Conjunction,
    DiagToken::Disjunction,
    DiagToken::Implication,
    DiagToken::MagicWand,
    DiagToken::StarStar,
];
impl<'db> Parsable<'db> for BinOp {
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token = peakable.expect_one_of(EXPECTED_OP)?;
        peakable.commit();
        match token.token(ctx.db) {
            Token::Conjunction => Ok(Self::Conjunction(Conjunction {
                span: token.span(ctx.db),
            })),
            Token::Disjunction => Ok(Self::Disjunction(Disjunction {
                span: token.span(ctx.db),
            })),
            Token::Implication => Ok(Self::Implication(Implication {
                span: token.span(ctx.db),
            })),
            Token::MagicWand => Ok(Self::AndSeparately(StarStar {
                span: token.span(ctx.db),
            })),
            Token::StarStar => Ok(Self::MagicWand(MagicWand {
                span: token.span(ctx.db),
            })),
            _ => unreachable!(),
        }
    }
}
