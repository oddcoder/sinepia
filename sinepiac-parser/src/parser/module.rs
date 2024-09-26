use super::Parsable;
use sinepia_ast::{functions::ItemFn, logic::HoareTriplet, Module, ModuleItem};
use sinepiac_diagnostics::{
    early_eof::EarlyEof, unexpected_token::UnexpectedToken, Token as DiagToken,
};
use sinepiac_lexer::Token;

const TERMINALS: &[DiagToken] = &[DiagToken::Exists, DiagToken::Forall, DiagToken::Ident];

impl<'db> Parsable<'db> for ModuleItem<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "ModuleItem::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> super::PResult<Self> {
        let mut peakable = ctx.peakable();
        let Some(token) = peakable.next() else {
            let src = peakable.ctx.tokens.file(peakable.ctx.db);
            let span = src.last_span(peakable.ctx.db);
            let err = EarlyEof::new(src, TERMINALS, span);
            return Err(err.into());
        };
        match token.token(peakable.ctx.db) {
            Token::Fn => Ok(Self::Fn(ItemFn::parse(ctx)?)),
            Token::Assuming => Ok(Self::HoareFn(HoareTriplet::parse(ctx)?)),
            tok => {
                let src = peakable.ctx.tokens.file(peakable.ctx.db);
                let err =
                    UnexpectedToken::new(src, TERMINALS, tok.into(), token.span(peakable.ctx.db));
                Err(err.into())
            }
        }
    }
}

impl<'db> Parsable<'db> for Module<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Module::parse")]
    fn parse(ctx: &mut crate::ParserCtx<'db>) -> super::PResult<Self> {
        let mut items = Vec::new();
        loop {
            let mut peakable = ctx.peakable();
            if peakable.next().is_none() {
                break;
            }
            items.push(ModuleItem::parse(ctx)?);
        }
        Ok(Self::new(ctx.db, items))
    }
}
