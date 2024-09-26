use crate::Token;
use logos::Logos;

const DATA1: &'static str = "assuming x > 0;\n";
const DATA2: &'static str = "fn div(x: u32, y: u32) -> u32 {\n";
const DATA3: &'static str = "   return y / x;\n";
const DATA4: &'static str = "} ergo return * x + y%x = x; //meh\n";
const DATA5: &'static str = "} ergo return * x + y%x = x; //meh";

#[test]
fn test_lexer1() {
    let lex = Token::lexer(DATA1);
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
    let lex = Token::lexer(DATA2);
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
    let lex = Token::lexer(DATA3);
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
    let lex = Token::lexer(DATA4);
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
    let lex = Token::lexer(DATA5);
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
