mod exists;
mod forall;
mod hoare;
mod probin;

use super::{PResult, Parsable};
use crate::ParserCtx;
use sinepia_ast::{
    literals::Ident,
    logic::{BinOp, Prop, PropBin, PropExist, PropForall},
};
use sinepiac_diagnostics::{
    early_eof::EarlyEof, unexpected_token::UnexpectedToken, Token as DiagToken,
};
use sinepiac_lexer::Token;

const TERMINALS: &[DiagToken] = &[DiagToken::Exists, DiagToken::Forall, DiagToken::Ident];
fn parse_expr_side<'db>(ctx: &mut ParserCtx<'db>) -> PResult<Prop<'db>> {
    let mut peakable = ctx.peakable();
    let Some(token) = peakable.next() else {
        let src = peakable.ctx.tokens.file(peakable.ctx.db);
        let span = src.last_span(peakable.ctx.db);
        let err = EarlyEof::new(src, TERMINALS, span);
        return Err(err.into());
    };
    match token.token(peakable.ctx.db) {
        Token::Exists => Ok(Prop::Exist(PropExist::parse(ctx)?)),
        Token::Forall => Ok(Prop::Forall(PropForall::parse(ctx)?)),
        Token::Ident => Ok(Prop::Ident(Ident::parse(ctx)?)),
        tok => {
            let src = peakable.ctx.tokens.file(peakable.ctx.db);
            let err = UnexpectedToken::new(src, TERMINALS, tok.into(), token.span(peakable.ctx.db));
            Err(err.into())
        }
    }
}

fn tail_curl<'db>(
    ctx: &mut ParserCtx<'db>,
    head: &mut Prop<'db>,
    tails: &mut Vec<(BinOp, Prop<'db>)>,
    prec: Precedence,
) {
    let Some((mut tail_op, mut tail_prop)) = tails.pop() else {
        return;
    };
    loop {
        if Precedence::from(tail_op) < prec {
            tails.push((tail_op, tail_prop));
            break;
        }
        let Some((op, prop)) = tails.pop() else {
            let bin = PropBin::new(ctx.db, Box::new(head.clone()), tail_op, Box::new(tail_prop));
            *head = Prop::Binary(bin);
            break;
        };
        tail_op = op;
        let bin = PropBin::new(ctx.db, Box::new(prop), tail_op, Box::new(tail_prop));
        tail_prop = Prop::Binary(bin);
    }
}
fn binary_parser<'db>(ctx: &mut ParserCtx<'db>, lhs: Prop<'db>) -> PResult<Prop<'db>> {
    let mut head = lhs;
    let mut tails = Vec::new();
    loop {
        let Ok(op) = BinOp::parse(ctx) else {
            break;
        };
        let expr = Prop::parse(ctx)?;
        let Some((last_op, _)) = tails.last() else {
            tails.push((op, expr));
            continue;
        };
        if Precedence::from(*last_op) < Precedence::from(op) {
            tails.push((op, expr));
        } else {
            tail_curl(ctx, &mut head, &mut tails, op.into());
        }
    }
    tail_curl(ctx, &mut head, &mut tails, Precedence::MIN);
    Ok(head)
}

impl<'db> Parsable<'db> for Prop<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Prop::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let lhs = parse_expr_side(ctx)?;
        binary_parser(ctx, lhs)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    MagicWand,
    AndSeparately,
    Disjunction,
    Conjunction,
    Implication,
}

impl Precedence {
    const MIN: Self = Precedence::MagicWand;
}
impl From<BinOp> for Precedence {
    fn from(value: BinOp) -> Self {
        match value {
            BinOp::Conjunction(_) => Precedence::Conjunction,
            BinOp::Disjunction(_) => Precedence::Disjunction,
            BinOp::Implication(_) => Precedence::Implication,
            BinOp::AndSeparately(_) => Precedence::AndSeparately,
            BinOp::MagicWand(_) => Precedence::MagicWand,
        }
    }
}
