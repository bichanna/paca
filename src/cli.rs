mod ir;
mod parse;
mod util;

use clap::*;
use log::{error, LevelFilter};
use std::fmt;
use std::fmt::Formatter;
use std::fs::read_to_string;

/// The log level options
#[derive(Default, ValueEnum, Clone, Debug, PartialEq)]
enum LogLevel {
    /// Display all errors.
    Error,
    /// Display all warnings and errors.
    Warn,
    /// Display all info messages.
    Info,
    /// Display all debug info messages.
    Debug,
    /// Trace the compilation of the program.
    Trace,
    /// Display no messages.
    #[default]
    None,
}

/// The target options to compile the given source code to.
#[derive(Default, ValueEnum, Clone, Debug, PartialEq)]
enum TargetType {
    /// Compile to the Paca assembly language (PASM).
    Pasm,
    /// Compile to the Paca intermediate representation (PIR).
    Pir,
    /// Compile to C code.
    #[default]
    C,
}

/// The source language options to compile.
#[derive(Default, ValueEnum, Clone, Debug, PartialEq)]
enum SourceType {
    /// Compile the Paca assembly language (PASM).
    Pasm,
    /// Compile the Paca intermediate representation (PIR).
    Pir,
    /// Compile Paca source code.
    #[default]
    Paca,
}

/// The argument parser for the CLI.
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    max_term_width = 90,
    about = "Paca - an experimental nibbler"
)]
struct CliArgs {
    /// The input file to the compiler.
    #[clap(value_parser)]
    input_file: String,

    /// The file to write the output of the compiler to.
    #[clap(short, long, value_parser, default_value = "paca-out")]
    output_file: String,

    /// The type of the source code to compile.
    #[clap(short, value_parser, default_value = "paca")]
    source_type: SourceType,

    /// The target language to compile to.
    #[clap(short, value_parser, default_value = "c")]
    target_type: TargetType,

    /// The log level to use.
    #[clap(short, long, value_parser, default_value = "info")]
    log_level: LogLevel,
}

/// The types of errors returned by the CLI.
enum Error {
    /// Error in reading source or writing generated code.
    IO(std::io::Error),
    /// Error parsing the source code.
    Parse(String),
    /// Error generating IR code.
    Pir(String), // TODO: Change the type to appropriate IR Error type.
    /// Error assembling input code.
    Pasm(String), // TODO: Change the type to appropriate PASM Error type.
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO error: {:?}", e),
            Error::Parse(e) => write!(f, "Parse error: {:?}", e),
            Error::Pir(e) => write!(f, "IR error: {:?}", e),
            Error::Pasm(e) => write!(f, "Assembly error: {:?}", e),
        }
    }
}
fn main() {
    // Parse the CLI arguments.
    let args = CliArgs::parse();
    let mut log_builder = env_logger::Builder::from_default_env();
    log_builder.format_timestamp(None);

    // Set the log level.
    log_builder.filter_level(match args.log_level {
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
        LogLevel::None => LevelFilter::Off,
    });

    log_builder.init();

    match read_to_string(&args.input_file).map_err(Error::IO) {
        Ok(file_content) => {
            todo!()
        }
        Err(e) => error!("Error reading file: {e:?}"),
    }
}
