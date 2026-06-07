use std::fmt::{Display, Formatter};
use crate::object::NsObject;
use crate::serializer::{CocoaSerializer, Key, Value};
use crate::writer::Base64Writer;
use plist::Uid;
use std::io::Write;

enum NsArchiver {
    NsKeyedArchiver,
}

enum ArchiveFormat {
    Base64,
    Binary
}

#[derive(Debug)]
enum ArchiveError {
    Plist(plist::Error),
}

impl Display for ArchiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::Plist(err) => write!(f, "{}", err),
        }
    }
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
            plist.to_writer_binary(Base64Writer::new(dst))
        } else {
            plist.to_writer_binary(dst)
        }.map_err(|err| ArchiveError::Plist(err))
    }
}

impl From<Value> for plist::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Integer(integer) => plist::Value::Integer(integer.into()),
            Value::Real(real) => plist::Value::Real(real),
            Value::String(string) => plist::Value::String(string),
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

trait Archiver {
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

struct ArchiverOptions {
    version: u32,
}
