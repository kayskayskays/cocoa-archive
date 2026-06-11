//! # CLI for cocoa-archive
//!
//! A command-line tool for constructing MacOS Cocoa archives.
//!
//! ## Usage
//!
//! ```bash
//! cocoa-archive --help
//! ```
//!
//! ## Examples
//!
//! ```sh
//! cocoa-archive color --red 1.0 --green 0.0 --blue 0.0 --alpha 1.0
//! ```
//! ```sh
//! cocoa-archive font --name "Helvetica" --size 12.0
//! ```
//! This binary crate is a simple wrapper around the [`cocoa_archive`] library crate.

use crate::cli::{Command, Format, GenericArgument, HELP, parse_command};
use cocoa_archive::{ArchiveError, ArchiveFormat, ArchiveTarget, Archiver, ArchiverVariant};
use std::error::Error;
use std::fmt::{Display, Formatter};

mod cli;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        eprintln!("try `cocoa-archive --help` for usage information");
        std::process::exit(2);
    }
}

fn run() -> Result<(), CliError> {
    let (cmd, generic_arguments) = parse_command().map_err(CliError::Parse)?;

    match cmd {
        Command::Version => {
            println!("cocoa-archive {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Command::Help => {
            print!("{HELP}");
            Ok(())
        }
        _ => run_archiver(cmd, generic_arguments),
    }
}

fn run_archiver(cmd: Command, generic_arguments: Vec<GenericArgument>) -> Result<(), CliError> {
    let archiver = ArchiverVariant::KeyedArchiver;

    let mut format = ArchiveFormat::Base64;
    for generic_argument in generic_arguments {
        match generic_argument {
            GenericArgument::Format(fmt) => {
                format = match fmt {
                    Format::Binary => ArchiveFormat::Binary,
                    Format::Base64 => ArchiveFormat::Base64,
                };
            }
        }
    }

    let target = match cmd {
        Command::Color {
            red,
            green,
            blue,
            alpha,
        } => ArchiveTarget::Color {
            red,
            green,
            blue,
            alpha,
        },
        Command::Font { name, size } => ArchiveTarget::Font { name, size },
        _ => unreachable!(),
    };

    archiver
        .archive(target, format, std::io::stdout().lock())
        .map_err(CliError::Archive)
}

#[derive(Debug)]
enum CliError {
    Archive(ArchiveError),
    Parse(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Archive(err) => {
                write!(f, "failed to create a Cocoa archive: {err}")
            }
            CliError::Parse(err) => write!(f, "failed to parse command-line arguments: {err}",),
        }
    }
}

impl Error for CliError {}
