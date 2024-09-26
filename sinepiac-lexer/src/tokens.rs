use logos::Logos;
use sinepiac_diagnostics::Token as DiagToken;
use sinepiac_span::{SourceFile, Span};
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("&")]
    And,
    #[token("&&")]
    AndAnd,
    #[token("&=")]
    AndEq,
    #[token("}")]
    BraceClose,
    #[token("{")]
    BraceOpen,
    #[token("^")]
    Caret,
    #[token("^=")]
    CaretEq,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("∧")]
    Conjunction,
    #[token("∨")]
    Disjunction,
    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token("∃")]
    Exists,
    #[token("∀")]
    Forall,
    #[token(">=")]
    Ge,
    #[token(">")]
    Gt,
    #[token("⟶")]
    Implication,
    #[token("<=")]
    Le,
    #[token("<")]
    Lt,
    #[token("--*")]
    MagicWand,
    #[token("-")]
    Minus,
    #[token("-=")]
    MinusEq,
    #[token("!=")]
    Ne,
    #[token("!")]
    Not,
    #[token("|")]
    Or,
    #[token("|=")]
    OrEq,
    #[token("||")]
    OrOr,
    #[token(")")]
    ParenClose,
    #[token("(")]
    ParenOpen,
    #[token("%")]
    Percent,
    #[token("%=")]
    PercentEq,
    #[token("+")]
    Plus,
    #[token("+=")]
    PlusEq,
    #[token("->")]
    RArrow,
    #[token(";")]
    Semi,
    #[token("<<")]
    Shl,
    #[token("<<=")]
    ShlEq,
    #[token(">>")]
    Shr,
    #[token(">>=")]
    ShrEq,
    #[token("/")]
    Slash,
    #[token("/=")]
    SlashEq,
    #[token("*")]
    Star,
    #[token("*=")]
    StarEq,
    #[token("**")]
    StarStar,
    #[token("assuming")]
    Assuming,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("else")]
    Else,
    #[token("ergo")]
    Ergo,
    #[token("false")]
    False,
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("let")]
    Let,
    #[token("loop")]
    Loop,
    #[token("proof")]
    Proof,
    #[token("qed")]
    Qed,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("while")]
    While,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9]*")]
    Ident,
    #[regex(r"[0-9]+")]
    Number,
    #[regex(r"[ \r\t\n]+")]
    Space,
    #[regex(r"//[^\n]*")]
    Comment,
}

impl From<Token> for DiagToken {
    fn from(val: Token) -> Self {
        match val {
            Token::And => DiagToken::And,
            Token::AndAnd => DiagToken::AndAnd,
            Token::AndEq => DiagToken::AndEq,
            Token::BraceClose => DiagToken::BraceClose,
            Token::BraceOpen => DiagToken::BraceOpen,
            Token::Caret => DiagToken::Caret,
            Token::CaretEq => DiagToken::CaretEq,
            Token::Colon => DiagToken::Colon,
            Token::Comma => DiagToken::Comma,
            Token::Conjunction => DiagToken::Conjunction,
            Token::Disjunction => DiagToken::Disjunction,
            Token::Eq => DiagToken::Eq,
            Token::EqEq => DiagToken::EqEq,
            Token::Exists => DiagToken::Exists,
            Token::Forall => DiagToken::Forall,
            Token::Ge => DiagToken::Ge,
            Token::Gt => DiagToken::Gt,
            Token::Implication => DiagToken::Implication,
            Token::Le => DiagToken::Le,
            Token::Lt => DiagToken::Lt,
            Token::MagicWand => DiagToken::MagicWand,
            Token::Minus => DiagToken::Minus,
            Token::MinusEq => DiagToken::MinusEq,
            Token::Ne => DiagToken::Ne,
            Token::Not => DiagToken::Not,
            Token::Or => DiagToken::Or,
            Token::OrEq => DiagToken::OrEq,
            Token::OrOr => DiagToken::OrOr,
            Token::ParenClose => DiagToken::ParenClose,
            Token::ParenOpen => DiagToken::ParenOpen,
            Token::Percent => DiagToken::Percent,
            Token::PercentEq => DiagToken::PercentEq,
            Token::Plus => DiagToken::Plus,
            Token::PlusEq => DiagToken::PlusEq,
            Token::RArrow => DiagToken::RArrow,
            Token::Semi => DiagToken::Semi,
            Token::Shl => DiagToken::Shl,
            Token::ShlEq => DiagToken::ShlEq,
            Token::Shr => DiagToken::Shr,
            Token::ShrEq => DiagToken::ShrEq,
            Token::Slash => DiagToken::Slash,
            Token::SlashEq => DiagToken::SlashEq,
            Token::Star => DiagToken::Star,
            Token::StarEq => DiagToken::StarEq,
            Token::StarStar => DiagToken::StarStar,
            Token::Assuming => DiagToken::Assuming,
            Token::Break => DiagToken::Break,
            Token::Continue => DiagToken::Continue,
            Token::Else => DiagToken::Else,
            Token::Ergo => DiagToken::Ergo,
            Token::Fn => DiagToken::Fn,
            Token::If => DiagToken::If,
            Token::Let => DiagToken::Let,
            Token::Loop => DiagToken::Loop,
            Token::Proof => DiagToken::Proof,
            Token::Qed => DiagToken::Qed,
            Token::Return => DiagToken::Return,
            Token::While => DiagToken::While,
            Token::Ident => DiagToken::Ident,
            Token::Number => DiagToken::Number,
            Token::Space => DiagToken::Space,
            Token::Comment => DiagToken::Comment,
            Token::False => DiagToken::False,
            Token::True => DiagToken::True,
        }
    }
}
#[salsa::tracked]
pub struct SpannedToken<'db> {
    pub token: Token,
    pub span: Span,
}

#[salsa::tracked]
pub struct Tokens<'db> {
    pub file: SourceFile,
    #[return_ref]
    pub tokens: Vec<SpannedToken<'db>>,
}
