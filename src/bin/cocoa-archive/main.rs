use crate::cli::Command::Font;
use crate::cli::parse_command;
use cocoa_archive::archiver::ArchiveFormat::Base64;
use cocoa_archive::archiver::NsArchiver::NsKeyedArchiver;
use cocoa_archive::archiver::{ArchiveError, Archiver};
use cocoa_archive::object::NsFont;
use cocoa_archive::serializer::NsFontSerializer;
use std::error::Error;
use std::fmt::{Display, Formatter};

mod cli;

#[derive(Debug)]
enum CliError {
    Archive(ArchiveError),
    Parse(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Archive(err) => write!(f, "{}", err),
            CliError::Parse(err) => write!(f, "{}", err),
        }
    }
}

impl Error for CliError {}

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
    }
}
