pub mod early_eof;
mod tokens;
mod traits;
pub mod unexpected_token;
pub mod unknown_token;

use codespan_reporting::diagnostic::Diagnostic as Diag;
pub use codespan_reporting::files::{Error, Files};
use early_eof::EarlyEof;
use sinepiac_span::SourceFile;
pub use tokens::*;
pub use traits::*;
use unexpected_token::UnexpectedToken;
use unknown_token::UnknownToken;

/// Diagnostic messages that are emitted using [`Diagnostics`].
#[derive(PartialEq, Eq, Hash)]
#[salsa::accumulator]
pub struct Diagnostic {
    inner: DiagnosticInner,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DiagnosticInner {
    UnknownToken(UnknownToken),
    UnexpectedToken(UnexpectedToken),
    EarlyEof(EarlyEof),
}

impl Diagnostic {
    pub fn as_diagnostic(&self) -> Diag<SourceFile> {
        match &self.inner {
            DiagnosticInner::UnknownToken(e) => e.as_diagnostic(),
            DiagnosticInner::UnexpectedToken(e) => e.as_diagnostic(),
            DiagnosticInner::EarlyEof(e) => e.as_diagnostic(),
        }
    }
}
