mod cli;

use clap::Parser;
use codespan_reporting::term::{emit, termcolor::ColorChoice};
use sinepiac_driver::Driver;
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

fn init_tracing(verbosity: u8) -> Result<(), ReturnStatus> {
    let verbosity = match verbosity {
        0 => LevelFilter::WARN,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        3 => LevelFilter::TRACE,
        _ => {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::WARN)
                .init();
            error!("Maximum verbosity level supported is 3");
            return Err(ReturnStatus::Error);
        }
    };
    let filter = EnvFilter::builder()
        .with_default_directive(verbosity.into())
        .from_env()
        .unwrap()
        .add_directive("salsa=error".parse().unwrap());
    tracing_subscriber::fmt().with_env_filter(filter).init();
    Ok(())
}

#[repr(i32)]
pub enum ReturnStatus {
    Success = 0,
    Error = 1,
}

pub fn run() -> ReturnStatus {
    let args = cli::Args::parse();
    if let Err(e) = init_tracing(args.verbosity) {
        return e;
    }
    info!("compiling {}", args.input);
    let mut stderr = codespan_reporting::term::termcolor::StandardStream::stderr(ColorChoice::Auto);
    let driver = match Driver::new(&args.input) {
        Ok(src) => src,
        Err(err) => {
            error!("Failed to compile {}: {}", args.input, err.to_string());
            return ReturnStatus::Error;
        }
    };
    if let Err(errors) = driver.build() {
        for error in errors {
            emit(
                &mut stderr,
                &Default::default(),
                &driver,
                &error.as_diagnostic(),
            )
            .unwrap();
        }
    };
    //let db = CalcDatabaseImpl::default();
    //let source_program = SourceProgram::new(&db, src);
    //compile(&db, source_program);
    //let diagnostics = compile::accumulated::<Diagnostic>(&db, source_program);
    //eprintln!("{diagnostics:?}");
    ReturnStatus::Success
}
