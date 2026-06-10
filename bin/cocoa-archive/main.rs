use crate::cli::{
    Command::{Color, Font},
    parse_command,
};
use cocoa_archive::{ArchiveError, ArchiveFormat, ArchiveTarget, Archiver, ArchiverVariant};
use std::error::Error;
use std::fmt::{Display, Formatter};

mod cli;

fn main() -> Result<(), CliError> {
    let stdout = std::io::stdout();
    let cmd = parse_command().map_err(CliError::Parse)?;

    let archiver = ArchiverVariant::KeyedArchiver;

    let target = match cmd {
        Color {
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
        Font { name, size } => ArchiveTarget::Font { name, size },
    };

    archiver
        .archive(target, ArchiveFormat::Base64, stdout.lock())
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
                write!(f, "Error occurred while creating a Cocoa archive: {}", err)
            }
            CliError::Parse(err) => write!(
                f,
                "Error occurred while parsing command-line arguments: {}",
                err
            ),
        }
    }
}

impl Error for CliError {}
