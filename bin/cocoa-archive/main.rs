use crate::cli::{parse_command, Command::{Color, Font}};
use cocoa_archive::{
    ArchiveError, ArchiveFormat::Base64, Archiver, NsArchiver::NsKeyedArchiver,
    NsColor,
    NsColorSerializer,
    NsFont,
    NsFontSerializer
};
use std::error::Error;
use std::fmt::{Display, Formatter};

mod cli;

fn main() -> Result<(), CliError> {
    let stdout = std::io::stdout();
    let cmd = parse_command().map_err(CliError::Parse)?;

    let archiver = NsKeyedArchiver;

    match cmd {
        Font { name, size } => {
            let object = NsFont::new(name, size);
            let serializer = NsFontSerializer;
            archiver
                .archive(object, serializer, Base64, stdout.lock())
                .map_err(CliError::Archive)
        }
        Color { red, green, blue, alpha } => {
            let object = NsColor::new(red, green, blue, alpha);
            let serializer = NsColorSerializer;
            archiver
                .archive(object, serializer, Base64, stdout.lock())
                .map_err(CliError::Archive)
        }
    }
}

#[derive(Debug)]
enum CliError {
    Archive(ArchiveError),
    Parse(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Archive(err) => write!(f, "Error occurred while creating a Cocoa archive: {}", err),
            CliError::Parse(err) => write!(f, "Error occurred while parsing command-line arguments: {}", err),
        }
    }
}

impl Error for CliError {}
