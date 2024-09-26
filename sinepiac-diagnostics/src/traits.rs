use crate::Diagnostic;
use codespan_reporting::diagnostic::{Diagnostic as Diag, Severity};
use sinepiac_span::SourceFile;

pub trait IntoDiagnostic: Into<Diagnostic> {
    /// A unique code which can be used when referring to this error.
    const CODE: &'static str;
    /// A simple message that is displayed with the error.
    const MESSAGE: &'static str;
    /// A verbose explanation of the error.
    const VERBOSE_DESCRIPTION: &'static str;
    /// The default severity.
    const SEVERITY: Severity;

    fn update_diag(&self, diag: Diag<SourceFile>) -> Diag<SourceFile>;

    fn as_diagnostic(&self) -> Diag<SourceFile> {
        let diag = Diag::new(Self::SEVERITY)
            .with_message(Self::MESSAGE)
            .with_code(Self::CODE);
        self.update_diag(diag)
    }
}
