use crate::Token;
use logos::Logos;

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
