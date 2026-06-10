use crate::ArchiveTarget::{Color, Font};
use crate::b64::writer::Base64Writer;
use crate::object::{NsColor, NsFont};
use crate::serializer::{
    CocoaKeyValueStore, CocoaSerializer, Key, NsColorSerializer, NsFontSerializer, Value,
};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

pub enum ArchiveFormat {
    Base64,
    Binary,
}

pub enum ArchiveTarget {
    Color {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    },
    Font {
        name: String,
        size: f32,
    },
}

pub enum ArchiverVariant {
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
                .map_err(ArchiveError::Plist)?;
            writer.finish().map_err(ArchiveError::Base64)
        } else {
            plist.to_writer_binary(writer).map_err(ArchiveError::Plist)
        }
    }
}

pub trait Archiver {
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

#[derive(Debug)]
pub enum ArchiveError {
    Plist(plist::Error),
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
