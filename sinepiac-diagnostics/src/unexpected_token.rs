use crate::{Diagnostic, DiagnosticInner, IntoDiagnostic, Token};
use codespan_reporting::diagnostic::{Diagnostic as Diag, Label, Severity};
use sinepiac_span::{SourceFile, Span};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnexpectedToken {
    file: SourceFile,
    expected_token: Vec<Token>,
    found: Token,
    loc: Span,
}

impl UnexpectedToken {
    pub fn new(file: SourceFile, expected: &[Token], unexpected: Token, loc: Span) -> Self {
        Self {
            file,
            loc,
            expected_token: expected.to_owned(),
            found: unexpected,
        }
    }
    fn message(&self) -> String {
        if self.expected_token.len() == 1 {
            format!(
                "Expected {}. Instead found {}",
                self.expected_token[0], self.found
            )
        } else {
            let list: Vec<_> = self.expected_token.iter().map(|t| format!("{t}")).collect();
            format!(
                "Expected one of: {}. Instead found {}",
                list.join(", "),
                self.found
            )
        }
    }
}

impl From<UnexpectedToken> for Diagnostic {
    fn from(value: UnexpectedToken) -> Self {
        Diagnostic {
            inner: DiagnosticInner::UnexpectedToken(value),
        }
    }
}

impl IntoDiagnostic for UnexpectedToken {
    const CODE: &'static str = "E002";
    const MESSAGE: &'static str = "Syntax error";
    const VERBOSE_DESCRIPTION: &'static str = "Unexpected token";
    const SEVERITY: Severity = Severity::Error;

    fn update_diag(&self, diag: Diag<SourceFile>) -> Diag<SourceFile> {
        let label = Label::primary(self.file, self.loc).with_message(self.message());
        diag.with_labels(vec![label])
    }
}
