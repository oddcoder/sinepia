use super::{PResult, Parsable};
use crate::ParserCtx;
use salsa::Update;
use sinepia_ast::punctuated::Punctuated;
use std::fmt::Display;

impl<'db, T, P> Parsable<'db> for Punctuated<T, P>
where
    T: Display + Update + Parsable<'db>,
    P: Display + Update + Parsable<'db>,
{
    #[sinepiac_instrument::instrument_parse]
    #[tracing::instrument(skip(ctx), name = "Punctuated::parse")]
    fn parse(ctx: &mut ParserCtx<'db>) -> PResult<Punctuated<T, P>> {
        let mut inner = Vec::new();
        let ret;
        loop {
            let Ok(data) = T::parse(ctx) else {
                ret = Ok(Self { inner, last: None });
                break;
            };
            let Ok(punct) = P::parse(ctx) else {
                ret = Ok(Self {
                    inner,
                    last: Some(Box::new(data)),
                });
                break;
            };
            inner.push((data, punct));
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::{test::TestDb, Parsable, ParserCtx};
    use sinepia_ast::{literals::Ident, punctuated::Punctuated, token::Comma};
    use sinepiac_lexer::lex_file;
    use sinepiac_span::SourceFile;
    use std::path::PathBuf;

    #[test]
    fn test_punctuated_one() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "foo".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        let x = Punctuated::<Ident, Comma>::parse(&mut ctx).unwrap();
        assert_eq!(x.len(), 1);
        assert!(!x.trailing())
    }

    #[test]
    fn test_punctuated_one_trailing() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "foo,".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        let x = Punctuated::<Ident, Comma>::parse(&mut ctx).unwrap();
        assert_eq!(x.len(), 1);
        assert!(x.trailing())
    }

    #[test]
    fn test_punctuated_multi() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "foo, bar, baz".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        let x = Punctuated::<Ident, Comma>::parse(&mut ctx).unwrap();
        assert_eq!(x.len(), 3);
        assert!(!x.trailing())
    }
    #[test]
    fn test_punctuated_multi_trailing() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "foo, bar, baz,".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        let x = Punctuated::<Ident, Comma>::parse(&mut ctx).unwrap();
        assert_eq!(x.len(), 3);
        assert!(x.trailing())
    }
    #[test]
    fn test_punctuated_empty() {
        let db = TestDb::default();
        let source = SourceFile::new(&db, PathBuf::default(), "if".to_owned());
        let tokens = lex_file(&db, source);
        let mut ctx = ParserCtx::new(&db, tokens);
        let x = Punctuated::<Ident, Comma>::parse(&mut ctx).unwrap();
        assert!(x.is_empty());
    }
}
