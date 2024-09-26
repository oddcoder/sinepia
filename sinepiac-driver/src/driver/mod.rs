mod diagnostics;

use crate::db::CompilerDb;
use sinepiac_diagnostics::Diagnostic;
use sinepiac_lexer::lex_file;
use sinepiac_span::SourceFile;
use std::{fs::read_to_string, io, path::Path};

pub struct Driver {
    source: SourceFile,
    db: CompilerDb,
}

impl Driver {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Driver> {
        let db = CompilerDb::default();
        let content = read_to_string(&path)?;
        let source = SourceFile::new(&db, path.as_ref().to_owned(), content);
        Ok(Self { db, source })
    }
    pub fn build(&self) -> Result<(), Vec<Diagnostic>> {
        let tokens = lex_file(&self.db, self.source);
        let diags: Vec<Diagnostic> = lex_file::accumulated(&self.db, self.source);
        if !diags.is_empty() {
            return Err(diags);
        }
        let _module = match sinepiac_parser::parse_module(&self.db, tokens) {
            Ok(m) => m,
            Err(e) => return Err(vec![e]),
        };
        unimplemented!()
    }
}
