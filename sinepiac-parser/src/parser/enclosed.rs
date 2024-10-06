use super::PResult;
use crate::{Parsable, ParserCtx};
use salsa::Update;
use sinepia_ast::enclosed::Enclosed;

impl<'db, O, T, C> Parsable<'db> for Enclosed<O, T, C>
where
    O: Update + Parsable<'db>,
    T: Update + Parsable<'db>,
    C: Update + Parsable<'db>,
{
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Self> {
        let open = O::parse(ctx)?;
        let inner = T::parse(ctx)?;
        let close = C::parse(ctx)?;
        Ok(Self { open, inner, close })
    }
}

#[cfg(test)]
mod test {
    use crate::{test::TestDb, Parsable, ParserCtx};
    use sinepia_ast::{
        enclosed::{Braced, Parenthesized},
        token::{Exists, Forall},
    };
    use sinepiac_lexer::lex_file;
    use sinepiac_span::SourceFile;
    use std::path::PathBuf;

    #[test]
    fn test_enclosed_parser() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "(∀) {∃}".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        Parenthesized::<Forall>::parse(&mut ctx).unwrap();
        Braced::<Exists>::parse(&mut ctx).unwrap();
    }
}
