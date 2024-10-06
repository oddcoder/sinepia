use crate::{lex_file, Token};
use logos::Logos;
use salsa::{Database, Storage};
use sinepiac_diagnostics::{unknown_token::UnknownToken, Diagnostic};
use sinepiac_span::{SourceFile, Span};
use std::path::PathBuf;

#[test]
fn test_lexer1() {
    let lex = Token::lexer("assuming x > 0;\n");
    let tokens: Vec<_> = lex.map(|x| x.unwrap()).collect();
    assert_eq!(
        tokens,
        &[
            Token::Assuming,
            Token::Space,
            Token::Ident,
            Token::Space,
            Token::Gt,
            Token::Space,
            Token::Number,
            Token::Semi,
            Token::Space,
        ]
    );
}

#[test]
fn test_lexer2() {
    let lex = Token::lexer("fn div(x: u32, y: u32) -> u32 {\n");
    let tokens: Vec<_> = lex.map(|x| x.unwrap()).collect();
    assert_eq!(
        tokens,
        &[
            Token::Fn,
            Token::Space,
            Token::Ident,
            Token::ParenOpen,
            Token::Ident,
            Token::Colon,
            Token::Space,
            Token::Ident,
            Token::Comma,
            Token::Space,
            Token::Ident,
            Token::Colon,
            Token::Space,
            Token::Ident,
            Token::ParenClose,
            Token::Space,
            Token::RArrow,
            Token::Space,
            Token::Ident,
            Token::Space,
            Token::BraceOpen,
            Token::Space,
        ]
    );
}

#[test]
fn test_lexer3() {
    let lex = Token::lexer("   return y / x;\n");
    let tokens: Vec<_> = lex.map(|x| x.unwrap()).collect();
    assert_eq!(
        tokens,
        &[
            Token::Space,
            Token::Return,
            Token::Space,
            Token::Ident,
            Token::Space,
            Token::Slash,
            Token::Space,
            Token::Ident,
            Token::Semi,
            Token::Space
        ]
    );
}

#[test]
fn test_lexer4() {
    let lex = Token::lexer("} ergo return * x + y%x = x; //meh\n");
    let tokens: Vec<_> = lex.map(|x| x.unwrap()).collect();
    assert_eq!(
        tokens,
        &[
            Token::BraceClose,
            Token::Space,
            Token::Ergo,
            Token::Space,
            Token::Return,
            Token::Space,
            Token::Star,
            Token::Space,
            Token::Ident,
            Token::Space,
            Token::Plus,
            Token::Space,
            Token::Ident,
            Token::Percent,
            Token::Ident,
            Token::Space,
            Token::Eq,
            Token::Space,
            Token::Ident,
            Token::Semi,
            Token::Space,
            Token::Comment,
            Token::Space
        ]
    );
}

#[test]
fn test_lexer5() {
    let lex = Token::lexer("} ergo return * x + y%x = x; //meh");
    let tokens: Vec<_> = lex.map(|x| x.unwrap()).collect();
    assert_eq!(
        tokens,
        &[
            Token::BraceClose,
            Token::Space,
            Token::Ergo,
            Token::Space,
            Token::Return,
            Token::Space,
            Token::Star,
            Token::Space,
            Token::Ident,
            Token::Space,
            Token::Plus,
            Token::Space,
            Token::Ident,
            Token::Percent,
            Token::Ident,
            Token::Space,
            Token::Eq,
            Token::Space,
            Token::Ident,
            Token::Semi,
            Token::Space,
            Token::Comment,
        ]
    );
}

#[test]
fn test_reserved() {
    let text = concat!(
        "& && &= } { ^ ^= : , ∧ ∨ = == ∃ ∀ >= > ⟶ <= < --* - -= != ! | |= || ) ( % %= ",
        "+ += -> ; << <<= >> >>= / /= * *= ** assuming break continue else ergo false fn ",
        "if let loop proof qed return true while",
    );
    let lex = Token::lexer(text);
    let tokens: Vec<_> = lex.map(|x| x.unwrap()).collect();
    assert_eq!(
        tokens,
        &[
            Token::And,
            Token::Space,
            Token::AndAnd,
            Token::Space,
            Token::AndEq,
            Token::Space,
            Token::BraceClose,
            Token::Space,
            Token::BraceOpen,
            Token::Space,
            Token::Caret,
            Token::Space,
            Token::CaretEq,
            Token::Space,
            Token::Colon,
            Token::Space,
            Token::Comma,
            Token::Space,
            Token::Conjunction,
            Token::Space,
            Token::Disjunction,
            Token::Space,
            Token::Eq,
            Token::Space,
            Token::EqEq,
            Token::Space,
            Token::Exists,
            Token::Space,
            Token::Forall,
            Token::Space,
            Token::Ge,
            Token::Space,
            Token::Gt,
            Token::Space,
            Token::Implication,
            Token::Space,
            Token::Le,
            Token::Space,
            Token::Lt,
            Token::Space,
            Token::MagicWand,
            Token::Space,
            Token::Minus,
            Token::Space,
            Token::MinusEq,
            Token::Space,
            Token::Ne,
            Token::Space,
            Token::Not,
            Token::Space,
            Token::Or,
            Token::Space,
            Token::OrEq,
            Token::Space,
            Token::OrOr,
            Token::Space,
            Token::ParenClose,
            Token::Space,
            Token::ParenOpen,
            Token::Space,
            Token::Percent,
            Token::Space,
            Token::PercentEq,
            Token::Space,
            Token::Plus,
            Token::Space,
            Token::PlusEq,
            Token::Space,
            Token::RArrow,
            Token::Space,
            Token::Semi,
            Token::Space,
            Token::Shl,
            Token::Space,
            Token::ShlEq,
            Token::Space,
            Token::Shr,
            Token::Space,
            Token::ShrEq,
            Token::Space,
            Token::Slash,
            Token::Space,
            Token::SlashEq,
            Token::Space,
            Token::Star,
            Token::Space,
            Token::StarEq,
            Token::Space,
            Token::StarStar,
            Token::Space,
            Token::Assuming,
            Token::Space,
            Token::Break,
            Token::Space,
            Token::Continue,
            Token::Space,
            Token::Else,
            Token::Space,
            Token::Ergo,
            Token::Space,
            Token::False,
            Token::Space,
            Token::Fn,
            Token::Space,
            Token::If,
            Token::Space,
            Token::Let,
            Token::Space,
            Token::Loop,
            Token::Space,
            Token::Proof,
            Token::Space,
            Token::Qed,
            Token::Space,
            Token::Return,
            Token::Space,
            Token::True,
            Token::Space,
            Token::While,
        ]
    );
}

#[salsa::db]
#[derive(Default)]
pub struct TestDb {
    storage: Storage<Self>,
}

#[salsa::db]
impl Database for TestDb {
    fn salsa_event(&self, _: &dyn Fn() -> salsa::Event) {}
}

#[test]
fn test_salsa() {
    let db = TestDb::default();
    let source = SourceFile::new(
        &db,
        PathBuf::default(),
        "hello γ if hello world 5 += σ ergo".to_owned(),
    );
    let tokens: Vec<_> = lex_file(&db, source)
        .tokens(&db)
        .iter()
        .map(|st| (st.token(&db), st.span(&db)))
        .collect();

    assert_eq!(
        tokens,
        &[
            (Token::Ident, Span { lo: 0, hi: 5 }), // hello
            (Token::Space, Span { lo: 5, hi: 6 }), // space
            // γ is s is skipped
            (Token::Space, Span { lo: 8, hi: 9 }),    // space
            (Token::If, Span { lo: 9, hi: 11 }),      // if
            (Token::Space, Span { lo: 11, hi: 12 }),  // space
            (Token::Ident, Span { lo: 12, hi: 17 }),  // hello
            (Token::Space, Span { lo: 17, hi: 18 }),  // space
            (Token::Ident, Span { lo: 18, hi: 23 }),  // world
            (Token::Space, Span { lo: 23, hi: 24 }),  // space
            (Token::Number, Span { lo: 24, hi: 25 }), // 5
            (Token::Space, Span { lo: 25, hi: 26 }),  // space
            (Token::PlusEq, Span { lo: 26, hi: 28 }), // +=
            (Token::Space, Span { lo: 28, hi: 29 }),  // space
            // σ is skipped
            (Token::Space, Span { lo: 31, hi: 32 }), // space
            (Token::Ergo, Span { lo: 32, hi: 36 })   // ergo
        ]
    );
    let diags: Vec<Diagnostic> = lex_file::accumulated(&db, source);
    assert_eq!(diags.len(), 2);
    assert_eq!(
        diags[0],
        UnknownToken::new(source, Span { lo: 6, hi: 8 }).into()
    );
    assert_eq!(
        diags[1],
        UnknownToken::new(source, Span { lo: 29, hi: 31 }).into()
    );
}
