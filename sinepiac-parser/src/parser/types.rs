use super::Parsable;
use crate::ParserCtx;
use sinepia_ast::{literals::Ident, types::Type};

impl<'db> Parsable<'db> for Type<'db> {
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Type::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> super::PResult<Self> {
        let ident = Ident::parse(ctx)?;
        Ok(Self::Ident(ident))
    }
}

#[cfg(test)]
mod test {
    use crate::{test::TestDb, Parsable, ParserCtx};
    use sinepia_ast::types::Type;
    use sinepiac_lexer::lex_file;
    use sinepiac_span::SourceFile;
    use std::path::PathBuf;

    #[test]
    fn test_type_parser() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "hello world".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        let Type::Ident(ident) = Type::parse(&mut ctx).unwrap();
        assert_eq!(ident.data(ctx.db), "hello");
        let Type::Ident(ident) = Type::parse(&mut ctx).unwrap();
        assert_eq!(ident.data(ctx.db), "world");
        Type::parse(&mut ctx).err().unwrap();
    }
}
