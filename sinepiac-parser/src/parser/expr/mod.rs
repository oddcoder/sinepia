mod binary;
mod block;
mod call;
mod ifelse;
mod lit;
mod loop_expr;
mod ret;
mod unary;
mod whi;

use super::{PResult, Parsable};
use crate::ParserCtx;
use sinepia_ast::{
    enclosed::{Enclosed, Parenthesized},
    expr::{
        BinOp, Block, Expr, ExprBinary, ExprCall, ExprIf, ExprLoop, ExprReturn, ExprUnary,
        ExprWhile, Lit,
    },
    literals::Ident,
    token::{Break, Continue},
};
use sinepiac_diagnostics::{
    early_eof::EarlyEof, unexpected_token::UnexpectedToken, Token as DiagToken,
};
use sinepiac_lexer::Token;

const TERMINALS: &[DiagToken] = &[
    DiagToken::Not,
    DiagToken::Minus,
    DiagToken::BraceOpen,
    DiagToken::If,
    DiagToken::True,
    DiagToken::False,
    DiagToken::Number,
    DiagToken::Loop,
    DiagToken::Return,
    DiagToken::While,
    DiagToken::Break,
    DiagToken::ParenOpen,
    DiagToken::Continue,
    DiagToken::Ident,
];

// <unop> expr
// if statement, loops, return, braek continue ...
// literals
fn parse_expr_side_terminals<'db>(ctx: &mut ParserCtx<'db>) -> PResult<Expr<'db>> {
    let mut peakable = ctx.peakable();
    let Some(token) = peakable.next() else {
        let src = peakable.ctx.tokens.file(peakable.ctx.db);
        let span = src.last_span(peakable.ctx.db);
        let err = EarlyEof::new(src, TERMINALS, span);
        return Err(err.into());
    };
    match token.token(peakable.ctx.db) {
        Token::BraceOpen => Ok(Expr::Block(Block::parse(ctx)?)),
        Token::If => Ok(Expr::If(ExprIf::parse(ctx)?)),
        Token::True | Token::False | Token::Number => Ok(Expr::Lit(Lit::parse(ctx)?)),
        Token::Loop => Ok(Expr::Loop(ExprLoop::parse(ctx)?)),
        Token::Return => Ok(Expr::Return(ExprReturn::parse(ctx)?)),
        Token::Minus | Token::Not => Ok(Expr::Unary(ExprUnary::parse(ctx)?)),
        Token::While => Ok(Expr::While(ExprWhile::parse(ctx)?)),
        Token::Break => Ok(Expr::Break(Break::parse(ctx)?)),
        Token::Continue => Ok(Expr::Continue(Continue::parse(ctx)?)),
        Token::ParenOpen => Ok(Expr::Tuple(Enclosed::parse(ctx)?)),
        Token::Ident => Ok(Expr::Ident(Ident::parse(ctx)?)),
        tok => {
            let src = peakable.ctx.tokens.file(peakable.ctx.db);
            let err = UnexpectedToken::new(src, TERMINALS, tok.into(), token.span(peakable.ctx.db));
            Err(err.into())
        }
    }
}

// same as parse_expr_side_terminals + calls
fn parse_expr_side<'db>(ctx: &mut ParserCtx<'db>) -> PResult<Expr<'db>> {
    let expr = parse_expr_side_terminals(ctx)?;
    let Ok(args) = Parenthesized::parse(ctx) else {
        return Ok(expr);
    };
    Ok(Expr::Call(ExprCall::new(ctx.db, Box::new(expr), args)))
}

fn tail_curl<'db>(
    ctx: &mut ParserCtx<'db>,
    head: &mut Expr<'db>,
    tails: &mut Vec<(BinOp, Expr<'db>)>,
    prec: Precedence,
) {
    let Some((mut tail_op, mut tail_exp)) = tails.pop() else {
        return;
    };
    loop {
        if Precedence::from(tail_op) < prec {
            tails.push((tail_op, tail_exp));
            break;
        }
        let Some((op, expr)) = tails.pop() else {
            let bin = ExprBinary::new(ctx.db, Box::new(head.clone()), tail_op, Box::new(tail_exp));
            *head = Expr::Binary(bin);
            break;
        };
        tail_op = op;
        let bin = ExprBinary::new(ctx.db, Box::new(expr), tail_op, Box::new(tail_exp));
        tail_exp = Expr::Binary(bin);
    }
}
fn binary_parser<'db>(ctx: &mut ParserCtx<'db>, lhs: Expr<'db>) -> PResult<Expr<'db>> {
    let mut head = lhs;
    let mut tails = Vec::new();
    loop {
        let Ok(op) = BinOp::parse(ctx) else {
            break;
        };
        let expr = Expr::parse(ctx)?;
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
impl<'db> Parsable<'db> for Expr<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Expr::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> PResult<Self> {
        let lhs = parse_expr_side(ctx)?;
        binary_parser(ctx, lhs)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    // = += -= *= /= %= &= |= ^= <<= >>=
    Assign,
    // ||
    Or,
    // &&
    And,
    // == != < > <= >=
    Compare,
    // |
    BitOr,
    // ^
    BitXor,
    // &
    BitAnd,
    // << >>
    Shift,
    // + -
    Sum,
    // * / %
    Product,
}
impl Precedence {
    const MIN: Self = Precedence::Assign;
}
impl From<BinOp> for Precedence {
    fn from(value: BinOp) -> Self {
        match value {
            BinOp::Mul(_) | BinOp::Div(_) | BinOp::Rem(_) => Precedence::Product,
            BinOp::Add(_) | BinOp::Sub(_) => Precedence::Sum,
            BinOp::Shl(_) | BinOp::Shr(_) => Precedence::Shift,
            BinOp::BitAnd(_) => Precedence::BitAnd,
            BinOp::BitXor(_) => Precedence::BitXor,
            BinOp::BitOr(_) => Precedence::BitOr,
            BinOp::EqEq(_)
            | BinOp::Lt(_)
            | BinOp::Le(_)
            | BinOp::Ne(_)
            | BinOp::Ge(_)
            | BinOp::Gt(_) => Precedence::Compare,
            BinOp::And(_) => Precedence::And,
            BinOp::Or(_) => Precedence::Or,
            BinOp::AddAssign(_)
            | BinOp::SubAssign(_)
            | BinOp::MulAssign(_)
            | BinOp::DivAssign(_)
            | BinOp::RemAssign(_)
            | BinOp::BitXorAssign(_)
            | BinOp::BitAndAssign(_)
            | BinOp::BitOrAssign(_)
            | BinOp::ShlAssign(_)
            | BinOp::ShrAssign(_)
            | BinOp::Eq(_) => Precedence::Assign,
        }
    }
}
