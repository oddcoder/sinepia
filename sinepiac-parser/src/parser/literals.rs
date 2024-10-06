use super::PResult;
use crate::{Parsable, ParserCtx};
use sinepia_ast::literals::{Ident, LitBool, LitInt};
use sinepiac_diagnostics::Token as DiagToken;
use sinepiac_lexer::Token;

impl<'db> Parsable<'db> for Ident<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Ident::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token = peakable.expect_one_of(&[DiagToken::Ident])?;
        peakable.commit();
        let src = ctx.tokens.file(ctx.db);
        let span = token.span(ctx.db);
        let name = src.at_span(ctx.db, span);
        Ok(Self::new(ctx.db, name.to_owned(), span))
    }
}

impl<'db> Parsable<'db> for LitBool<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "LitBool::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token = peakable.expect_one_of(&[DiagToken::True, DiagToken::False])?;
        peakable.commit();
        let lit = match token.token(ctx.db) {
            Token::True => true,
            Token::False => false,
            _ => unreachable!(), // thanks to expect_one_of method
        };
        let ret = LitBool::new(ctx.db, lit, token.span(ctx.db));
        Ok(ret)
    }
}

impl<'db> Parsable<'db> for LitInt<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "LitInt::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let mut peakable = ctx.peakable();
        let token = peakable.expect_one_of(&[DiagToken::Number])?;
        peakable.commit();
        let src = ctx.tokens.file(ctx.db);
        let span = token.span(ctx.db);
        let lit = src.at_span(ctx.db, span);
        Ok(Self::new(ctx.db, lit.to_owned(), span))
    }
}

#[cfg(test)]
mod test {
    use crate::{test::TestDb, Parsable, ParserCtx};
    use sinepia_ast::literals::{Ident, LitBool, LitInt};
    use sinepiac_lexer::lex_file;
    use sinepiac_span::SourceFile;
    use std::path::PathBuf;

    #[test]
    fn test_ident_parser() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "hello world".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        let ident = Ident::parse(&mut ctx).unwrap();
        assert_eq!(ident.data(ctx.db), "hello");
        let ident = Ident::parse(&mut ctx).unwrap();
        assert_eq!(ident.data(ctx.db), "world");
        Ident::parse(&mut ctx).err().unwrap();
    }
    #[test]
    fn test_bool_parser() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "true false".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        let lit = LitBool::parse(&mut ctx).unwrap();
        assert!(lit.data(ctx.db));
        let lit = LitBool::parse(&mut ctx).unwrap();
        assert!(!lit.data(ctx.db));
        LitBool::parse(&mut ctx).err().unwrap();
    }
    #[test]
    fn test_number_parser() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "123456 \n1234  ".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        let lit = LitInt::parse(&mut ctx).unwrap();
        assert_eq!(lit.data(ctx.db), "123456");
        let lit = LitInt::parse(&mut ctx).unwrap();
        assert_eq!(lit.data(ctx.db), "1234");
        LitBool::parse(&mut ctx).err().unwrap();
    }
}
