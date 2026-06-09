use crate::b64::writer::Base64Writer;
use crate::object::NsObject;
use crate::serializer::{CocoaSerializer, Key, Value};
use plist::Uid;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

pub enum NsArchiver {
    NsKeyedArchiver,
}

pub enum ArchiveFormat {
    Base64,
    Binary,
}

impl NsArchiver {
    fn ns_keyed_archive<T, U>(
        &self,
        object: T,
        serializer: U,
        format: ArchiveFormat,
        dst: impl Write,
    ) -> Result<(), ArchiveError>
    where
        T: NsObject,
        U: CocoaSerializer<T>,
    {
        let mut store = serializer.serialize(object);
        store.insert(
            Key::Archiver,
            Value::String(String::from("NSKeyedArchiver")),
        );
        store.insert(Key::Version, Value::Integer(100000));

        let root = Value::Dictionary(store);
        let plist = plist::Value::from(root);

        if let ArchiveFormat::Base64 = format {
            let mut writer = Base64Writer::new(dst);
            plist
                .to_writer_binary(&mut writer)
                .map_err(ArchiveError::Plist)?;
            writer.finish().map_err(ArchiveError::Base64)
        } else {
            plist.to_writer_binary(dst).map_err(ArchiveError::Plist)
        }
    }
}

pub trait Archiver {
    fn archive<T, U>(
        &self,
        object: T,
        serializer: U,
        format: ArchiveFormat,
        dst: impl Write,
    ) -> Result<(), ArchiveError>
    where
        T: NsObject,
        U: CocoaSerializer<T>;
}

impl Archiver for NsArchiver {
    fn archive<T, U>(
        &self,
        object: T,
        serializer: U,
        format: ArchiveFormat,
        dst: impl Write,
    ) -> Result<(), ArchiveError>
    where
        T: NsObject,
        U: CocoaSerializer<T>,
    {
        match self {
            NsArchiver::NsKeyedArchiver => self.ns_keyed_archive(object, serializer, format, dst),
        }
    }
}

struct ArchiverOptions {
    version: u32,
}

impl From<Value> for plist::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Integer(integer) => plist::Value::Integer(integer.into()),
            Value::Real(real) => plist::Value::Real(real),
            Value::String(string) => plist::Value::String(string),
            Value::Data(data) => plist::Value::Data(data),
            Value::Ref(uid) => plist::Value::Uid(Uid::new(uid)),
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
            ArchiveError::Plist(err) => write!(f, "{}", err),
            ArchiveError::Base64(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ArchiveError {}
