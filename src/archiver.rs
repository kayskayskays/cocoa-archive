use crate::b64::writer::Base64Writer;
use crate::object::{NsColor, NsFont};
use crate::serializer::{
    CocoaKeyValueStore, CocoaSerializer, Key, NsColorSerializer, NsFontSerializer, Value,
};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

/// Supported archive formats.
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub enum ArchiveFormat {
    /// The binary property list format.
    #[default]
    Binary,
    /// The base64-encoded binary property list format.
    Base64,
}

/// Supported Cocoa objects that can be archived.
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

/// The various [`Archiver`] implementations provided by this crate.
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub enum ArchiverVariant {
    /// An archiver that archives property list data as key-value pairs; this
    /// archiver corresponds to the Apple `NSKeyedArchiver` archiver.
    #[default]
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
                .map_err(|err| ArchiveError::PropertyList(err.to_string()))?;
            writer.finish().map_err(ArchiveError::Base64)
        } else {
            plist
                .to_writer_binary(writer)
                .map_err(|err| ArchiveError::PropertyList(err.to_string()))
        }
    }
}

/// Archives supported Cocoa objects.
pub trait Archiver {
    /// Archives an [`ArchiveTarget`] using the specified [`ArchiveFormat`].
    ///
    /// The serialized archive is written to `writer`.
    ///
    /// # Example
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
    ///
    /// let mut writer = Vec::new();
    ///
    /// ArchiverVariant::KeyedArchiver
    ///     .archive(target, ArchiveFormat::Base64, &mut writer)
    ///     .unwrap();
    ///
    /// assert!(!writer.is_empty());
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
            ArchiveTarget::Color {
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
            ArchiveTarget::Font { name, size } => {
                NsFontSerializer.serialize(NsFont::new(name, size))
            }
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

/// Errors that may occur while attempting to archive a Cocoa object.
#[derive(Debug)]
#[non_exhaustive]
pub enum ArchiveError {
    /// Failed while serializing a property list archive.
    PropertyList(String),
    /// Failed while serializing a base64-encoded property list archive.
    Base64(std::io::Error),
}

impl Display for ArchiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::PropertyList(err) => {
                write!(f, "failed while serializing a property list archive: {err}")
            }
            ArchiveError::Base64(err) => write!(
                f,
                "failed while serializing a base64-encoded property list archive: {err}",
            ),
        }
    }
}

impl Error for ArchiveError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    #[ignore = "requires macOS toolchain, including Swift and the AppKit/Foundation libraries "]
    fn macos_compatability_generated_color_archive_decodes_with_apple_unarchiver() {
        let archive_target = ArchiveTarget::Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        };

        invoke_swift_runner(archive_target, "color");
    }

    #[test]
    #[ignore = "requires macOS toolchain, including Swift and the AppKit/Foundation libraries "]
    fn macos_compatability_generated_font_archive_decodes_with_apple_unarchiver() {
        let archive_target = ArchiveTarget::Font {
            name: "Helvetica".to_string(),
            size: 12.0,
        };

        invoke_swift_runner(archive_target, "font");
    }

    fn invoke_swift_runner(target: ArchiveTarget, arg: &str) {
        let mut writer = Vec::new();
        ArchiverVariant::KeyedArchiver
            .archive(target, ArchiveFormat::Base64, &mut writer)
            .expect("failed to archive NSColor");

        let status = Command::new("swift")
            .arg("tests/unarchive.swift")
            .arg(arg)
            .arg(String::from_utf8(writer).expect("failed to convert base64 archive to string"))
            .status()
            .expect("failed to invoke Swift test runner");

        assert!(status.success());
    }
}
