use super::ParserCtx;
use crate::test::TestDb;
use sinepiac_diagnostics::{
    early_eof::EarlyEof, unexpected_token::UnexpectedToken, Token as DiagToken,
};
use sinepiac_lexer::{lex_file, Token};
use sinepiac_span::{SourceFile, Span};
use std::path::PathBuf;

#[test]
fn test_drop_peakable() {
    let db = TestDb::default();
    let source = SourceFile::new(
        &db,
        PathBuf::default(),
        "hello if hello world 5 += ergo".to_owned(),
    );
    let tokens = lex_file(&db, source);
    let mut ctx = ParserCtx::new(&db, tokens);
    assert_eq!(ctx.pos, 0);
    let mut peakable = ctx.peakable();
    for _ in 0..5 {
        peakable.next();
    }
    assert_eq!(ctx.pos, 0);
}

#[test]
fn test_commit_peakable() {
    let db = TestDb::default();
    let source = SourceFile::new(
        &db,
        PathBuf::default(),
        "hello if hello world 5 += ergo".to_owned(),
    );
    let tokens = lex_file(&db, source);
    let mut ctx = ParserCtx::new(&db, tokens);
    assert_eq!(ctx.pos, 0);
    let mut peakable = ctx.peakable();
    peakable.next();
    peakable.commit();
    assert_eq!(ctx.pos, 1);
    let mut peakable = ctx.peakable();
    peakable.next();
    peakable.commit();
    assert_eq!(ctx.pos, 3);
    let mut peakable = ctx.peakable();
    for _ in 0..3 {
        peakable.next();
    }
    peakable.commit();
    assert_eq!(ctx.pos, 9);
}

#[test]
fn test_peakable() {
    let db = TestDb::default();
    let source = SourceFile::new(&db, PathBuf::default(), "hello my hello".to_owned());
    let tokens = lex_file(&db, source);
    let mut ctx = ParserCtx::new(&db, tokens);
    let mut peakable = ctx.peakable();
    for _ in 0..3 {
        let Some(tok) = peakable.next() else {
            panic!("Expected token found nothing")
        };
        assert_eq!(tok.token(&db), Token::Ident);
    }
    assert_eq!(peakable.next(), None);
    peakable.commit();
    let mut peakable = ctx.peakable();
    assert_eq!(peakable.next(), None);
    peakable.commit();
}

#[test]
fn test_peakable_expect() {
    let db = TestDb::default();
    let source = SourceFile::new(&db, PathBuf::default(), "hello my hello".to_owned());
    let tokens = lex_file(&db, source);
    let mut ctx = ParserCtx::new(&db, tokens);
    let mut peakable = ctx.peakable();
    peakable.expect_one_of(&[DiagToken::Ident]).unwrap();
    peakable.commit();
    assert_eq!(ctx.pos, 1);
    let mut peakable = ctx.peakable();
    let err = peakable.expect_one_of(&[DiagToken::If]).err().unwrap();
    assert_eq!(
        err,
        UnexpectedToken::new(
            source,
            &[DiagToken::If],
            DiagToken::Ident,
            Span { lo: 6, hi: 8 }
        )
        .into()
    );
    peakable.commit();
    assert_eq!(ctx.pos, 1);
    let mut peakable = ctx.peakable();
    peakable.next();
    peakable.next();
    let err = peakable.expect_one_of(&[DiagToken::If]).err().unwrap();
    assert_eq!(
        err,
        EarlyEof::new(source, &[DiagToken::If], Span { lo: 13, hi: 13 }).into()
    );
}
