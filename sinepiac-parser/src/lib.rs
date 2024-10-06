mod context;
mod parser;
#[cfg(test)]
pub mod test;
pub use context::ParserCtx;
pub use parser::{PResult, Parsable};
use salsa::{plumbing::attach, Database};
use sinepia_ast::Module;
use sinepiac_diagnostics::Diagnostic;
use sinepiac_lexer::Tokens;
use tracing::{debug, instrument};

#[instrument(skip(db, tokens), name = "sinepiac_parser::parse_module")]
#[salsa::tracked]
pub fn parse_module<'db>(
    db: &'db dyn Database,
    tokens: Tokens<'db>,
) -> Result<Module<'db>, Diagnostic> {
    let mut ctx = ParserCtx::new(db, tokens);
    Module::parse(&mut ctx)
        .inspect(|module| {
            attach(db, || {
                debug!("Module = {}", module);
            })
        })
        .inspect_err(|e| debug!("Returning an Error, {e:?}"))
}
