use crate::{PResult, Parsable};
use sinepia_ast::{
    enclosed::Braced,
    expr::{Block, Expr, Local, Stmt},
    literals::Ident,
    token::{Eq, Let, Semi},
};
use sinepiac_diagnostics::Token as DiagToken;

impl<'db> Parsable<'db> for Block<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Block::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let stmts = Braced::parse(ctx)?;
        Ok(Block::new(ctx.db, stmts))
    }
}

impl<'db> Parsable<'db> for Local<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Local::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let let_token = Let::parse(ctx)?;
        let ident = Ident::parse(ctx)?;
        let eq_token = Eq::parse(ctx)?;
        let expr = Expr::parse(ctx)?;
        let semi_token = Semi::parse(ctx)?;
        Ok(Self::new(
            ctx.db, let_token, ident, eq_token, expr, semi_token,
        ))
    }
}

impl<'db> Parsable<'db> for Stmt<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Stmt::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        if peakable.expect_one_of(&[DiagToken::Let]).is_ok() {
            return Ok(Stmt::Local(Local::parse(ctx)?));
        }
        let expr = Expr::parse(ctx)?;
        let mut peakable = ctx.peakable();
        if let Ok(semi) = peakable.expect_one_of(&[DiagToken::Semi]) {
            peakable.commit();
            Ok(Self::Expr(
                expr,
                Some(Semi {
                    span: semi.span(ctx.db),
                }),
            ))
        } else {
            Ok(Self::Expr(expr, None))
        }
    }
}
