use crate::{Diagnostic, DiagnosticInner, IntoDiagnostic, Token};
use codespan_reporting::diagnostic::{Diagnostic as Diag, Label, Severity};
use sinepiac_span::{SourceFile, Span};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EarlyEof {
    file: SourceFile,
    expected_token: Vec<Token>,
    loc: Span,
}

impl EarlyEof {
    pub fn new(file: SourceFile, expected: &[Token], loc: Span) -> Self {
        Self {
            file,
            expected_token: expected.to_owned(),
            loc,
        }
    }
    fn message(&self) -> String {
        if self.expected_token.len() == 1 {
            format!("Expected {}.", self.expected_token[0])
        } else {
            let list: Vec<_> = self.expected_token.iter().map(|t| format!("{t}")).collect();
            format!("Expected one of: {}.", list.join(", "),)
        }
    }
}

impl From<EarlyEof> for Diagnostic {
    fn from(value: EarlyEof) -> Self {
        Diagnostic {
            inner: DiagnosticInner::EarlyEof(value),
        }
    }
}

impl IntoDiagnostic for EarlyEof {
    const CODE: &'static str = "E003";
    const MESSAGE: &'static str = "Syntax error";
    const VERBOSE_DESCRIPTION: &'static str = "Unexpected early EOF";
    const SEVERITY: Severity = Severity::Error;

    fn update_diag(&self, diag: Diag<SourceFile>) -> Diag<SourceFile> {
        let label = Label::primary(self.file, self.loc).with_message(self.message());
        diag.with_labels(vec![label])
    }
}
