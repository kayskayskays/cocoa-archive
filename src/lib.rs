//! # cocoa-archive
//!
//! A tool for constructing MacOS Cocoa archives.
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! cocoa-archive = "0.1"
//! ```
//! After which, put the following in your crate root:
//!
//! ```rust
//! use cocoa_archive;
//! ```

#![deny(warnings, missing_debug_implementations, missing_copy_implementations)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod archiver;
mod b64;
mod object;
mod serializer;

pub use archiver::{ArchiveError, ArchiveFormat, ArchiveTarget, Archiver, ArchiverVariant};
