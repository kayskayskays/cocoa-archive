use crate::ArchiveTarget::{Color, Font};
use crate::b64::writer::Base64Writer;
use crate::object::{NsColor, NsFont};
use crate::serializer::{
    CocoaKeyValueStore, CocoaSerializer, Key, NsColorSerializer, NsFontSerializer, Value,
};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

/// This enum defines the various supported archive formats.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ArchiveFormat {
    /// The base64 encoded binary plist format.
    Base64,
    /// The binary plist format.
    Binary,
}

/// This enum defines the various supported Cocoa objects that can be archived.
#[derive(Debug)]
#[non_exhaustive]
pub enum ArchiveTarget {
    /// An archive target for a color object; this maps to the Cocoa `NSColor` class.
    Color {
        /// The red component of the color.
        red: f32,
        /// The green component of the color.
        green: f32,
        /// The blue component of the color.
        blue: f32,
        /// The alpha component of the color.
        alpha: f32,
    },
    /// An archive target for a font object; this maps to the Cocoa `NSFont` class.
    Font {
        /// The name of the font.
        name: String,
        /// The size of the font.
        size: f32,
    },
}

/// This enum defines the various [`Archiver`] implementations provided by this crate.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ArchiverVariant {
    /// An archiver that archives plist data as key-value pairs; this archiver maps to the Cocoa
    /// `NSKeyedArchiver` archiver instance.
    KeyedArchiver,
}

impl ArchiverVariant {
    fn ns_keyed_archive(
        &self,
        mut store: CocoaKeyValueStore,
        format: ArchiveFormat,
        writer: impl Write,
    ) -> Result<(), ArchiveError> {
        store.insert(
            Key::Archiver,
            Value::String(String::from("NSKeyedArchiver")),
        );
        store.insert(Key::Version, Value::Integer(100000));

        let root = Value::Dictionary(store);
        let plist = plist::Value::from(root);

        if let ArchiveFormat::Base64 = format {
            let mut writer = Base64Writer::new(writer);
            plist
                .to_writer_binary(&mut writer)
                .map_err(|err| ArchiveError::Plist(err.to_string()))?;
            writer.finish().map_err(ArchiveError::Base64)
        } else {
            plist
                .to_writer_binary(writer)
                .map_err(|err| ArchiveError::Plist(err.to_string()))
        }
    }
}

/// This trait defines the interface for archiving supported Cocoa objects present in the
/// [`ArchiveTarget`] enum.
pub trait Archiver {
    /// Archives an [`ArchiveTarget`] using the specified [`ArchiveFormat`]. The
    /// serialized target will be written to the provided `writer`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cocoa_archive::*;
    ///
    /// let target = ArchiveTarget::Color {
    ///     red: 1.0,
    ///     green: 0.0,
    ///     blue: 0.0,
    ///     alpha: 1.0
    /// };
    /// let format = ArchiveFormat::Base64;
    /// let writer = std::io::stdout();
    ///
    /// ArchiverVariant::KeyedArchiver.archive(target, format, writer.lock())
    ///     .map_err(|err| {
    ///         match err {
    ///             ArchiveError::Plist(msg) => panic!("Plist error: {}", msg),
    ///             ArchiveError::Base64(err) => panic!("Base64 error: {}", err),
    ///         }
    ///     });
    /// ```
    fn archive(
        &self,
        target: ArchiveTarget,
        format: ArchiveFormat,
        writer: impl Write,
    ) -> Result<(), ArchiveError>;
}

impl Archiver for ArchiverVariant {
    fn archive(
        &self,
        target: ArchiveTarget,
        format: ArchiveFormat,
        writer: impl Write,
    ) -> Result<(), ArchiveError> {
        let store = match target {
            Color {
                red,
                green,
                blue,
                alpha,
            } => NsColorSerializer.serialize(NsColor {
                red,
                green,
                blue,
                alpha,
            }),
            Font { name, size } => NsFontSerializer.serialize(NsFont::new(name, size)),
        };

        match self {
            ArchiverVariant::KeyedArchiver => self.ns_keyed_archive(store, format, writer),
        }
    }
}

impl From<Value> for plist::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Integer(integer) => plist::Value::Integer(integer.into()),
            Value::Real(real) => plist::Value::Real(real),
            Value::String(string) => plist::Value::String(string),
            Value::Data(data) => plist::Value::Data(data),
            Value::Ref(uid) => plist::Value::Uid(plist::Uid::new(uid)),
            Value::Array(array) => plist::Value::Array(array.into_iter().map(Self::from).collect()),
            Value::Dictionary(store) => plist::Value::Dictionary(
                store
                    .into_iter()
                    .map(|(key, value)| (String::from(key), Self::from(value)))
                    .collect(),
            ),
        }
    }
}

/// This enum is a wrapper around errors that may occur while attempting to archive a Cocoa object.
#[derive(Debug)]
#[non_exhaustive]
pub enum ArchiveError {
    /// Indicative of an error that occurred while serializing a plist.
    Plist(String),
    /// Indicative of an error that occurred during base64 encoding of an archive.
    Base64(std::io::Error),
}

impl Display for ArchiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::Plist(err) => {
                write!(f, "Encountered error while serializing plist: {}", err)
            }
            ArchiveError::Base64(err) => write!(
                f,
                "Encountered error while serializing base64 encoded plist data: {}",
                err
            ),
        }
    }
}

impl Error for ArchiveError {}
