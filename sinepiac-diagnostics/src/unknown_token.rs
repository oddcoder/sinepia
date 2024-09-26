use crate::{Diagnostic, DiagnosticInner, IntoDiagnostic};
use codespan_reporting::diagnostic::{Diagnostic as Diag, Label, Severity};
use sinepiac_span::{SourceFile, Span};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownToken {
    file: SourceFile,
    loc: Span,
}

impl UnknownToken {
    pub fn new(file: SourceFile, loc: Span) -> Self {
        Self { file, loc }
    }
}

impl From<UnknownToken> for Diagnostic {
    fn from(value: UnknownToken) -> Self {
        Diagnostic {
            inner: DiagnosticInner::UnknownToken(value),
        }
    }
}

impl IntoDiagnostic for UnknownToken {
    const CODE: &'static str = "E001";
    const MESSAGE: &'static str = "Syntax error";
    const VERBOSE_DESCRIPTION: &'static str = "Unkown token";
    const SEVERITY: Severity = Severity::Error;

    fn update_diag(&self, diag: Diag<SourceFile>) -> Diag<SourceFile> {
        let label = Label::primary(self.file, self.loc);
        diag.with_labels(vec![label])
    }
}
