use crate::{PResult, Parsable, ParserCtx};
use sinepia_ast::token::*;
use sinepiac_diagnostics::Token as DiagToken;

macro_rules! parse_token {
    ($token: ident) => {
        impl<'db> Parsable<'db> for $token {
            fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
                let mut peakable = ctx.peakable();
                let token = peakable.expect_one_of(&[DiagToken::$token])?;
                peakable.commit();
                Ok(Self {
                    span: token.span(ctx.db),
                })
            }
        }
    };
}

parse_token!(And);
parse_token!(AndAnd);
parse_token!(AndEq);
parse_token!(StarStar);
parse_token!(Assuming);
parse_token!(BraceClose);
parse_token!(BraceOpen);
parse_token!(Break);
parse_token!(Caret);
parse_token!(CaretEq);
parse_token!(Colon);
parse_token!(Comma);
parse_token!(Conjunction);
parse_token!(Continue);
parse_token!(Disjunction);
parse_token!(Else);
parse_token!(Eq);
parse_token!(EqEq);
parse_token!(Ergo);
parse_token!(Exists);
parse_token!(Fn);
parse_token!(Forall);
parse_token!(Ge);
parse_token!(Gt);
parse_token!(If);
parse_token!(Implication);
parse_token!(Le);
parse_token!(Let);
parse_token!(Lt);
parse_token!(Loop);
parse_token!(MagicWand);
parse_token!(Minus);
parse_token!(MinusEq);
parse_token!(Ne);
parse_token!(Not);
parse_token!(Or);
parse_token!(OrEq);
parse_token!(OrOr);
parse_token!(ParenClose);
parse_token!(ParenOpen);
parse_token!(Percent);
parse_token!(PercentEq);
parse_token!(Plus);
parse_token!(PlusEq);
parse_token!(Proof);
parse_token!(Qed);
parse_token!(RArrow);
parse_token!(Return);
parse_token!(Semi);
parse_token!(Shl);
parse_token!(ShlEq);
parse_token!(Shr);
parse_token!(ShrEq);
parse_token!(Slash);
parse_token!(SlashEq);
parse_token!(Star);
parse_token!(StarEq);
parse_token!(While);
