#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod archiver;
mod b64;
mod object;
mod serializer;

pub use archiver::{ArchiveError, ArchiveFormat, ArchiveTarget, Archiver, ArchiverVariant};
