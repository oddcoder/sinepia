use crate::{PResult, Parsable};
use sinepia_ast::{
    expr::{Expr, ExprUnary, UnOp},
    token::{Minus, Not},
};
use sinepiac_diagnostics::Token as DiagToken;
use sinepiac_lexer::Token;

impl<'db> Parsable<'db> for ExprUnary<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ExprUnary::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let unop = UnOp::parse(ctx)?;
        let expr = Expr::parse(ctx)?;
        Ok(Self::new(ctx.db, unop, Box::new(expr)))
    }
}

pub const UNARY_OPS: &[DiagToken] = &[DiagToken::Not, DiagToken::Minus];

impl<'db> Parsable<'db> for UnOp {
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token = peakable.expect_one_of(UNARY_OPS)?;
        peakable.commit();
        match token.token(ctx.db) {
            Token::Not => Ok(Self::Not(Not {
                span: token.span(ctx.db),
            })),
            Token::Minus => Ok(Self::Neg(Minus {
                span: token.span(ctx.db),
            })),
            _ => unreachable!(),
        }
    }
}
