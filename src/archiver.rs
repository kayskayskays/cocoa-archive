use std::collections::HashMap;
use crate::serializer::CocoaSerializer;

enum NsArchiver {
    NsKeyedArchiver,
}

pub(crate) type ArchiverKeyValueStore = HashMap<String, crate::serializer::Value>;

impl NsArchiver {
    fn ns_keyed_archive<T, U>(&self, object: T, serializer: U) -> CocoaArchive
    where
        U: CocoaSerializer<T>
    {
        let mut store = ArchiverKeyValueStore::new();
        serializer.serialize(object, &mut store);
        todo!()
    }
}

impl Archiver for NsArchiver {
    fn archive<T, U>(&self, object: T, serializer: U) -> CocoaArchive
    where
        U: CocoaSerializer<T>
    {
        match self {
            NsArchiver::NsKeyedArchiver => self.ns_keyed_archive(object, serializer),
        }
    }
}

trait Archiver {
    fn archive<T, U>(&self, object: T, serializer: U) -> CocoaArchive
    where
        U: CocoaSerializer<T>;
}

struct ArchiverOptions {
    version: u32
}

struct CocoaArchive {
    name: String,
    version: u32,
}

impl CocoaArchive {
    fn new(name: String, version: u32) -> Self {
        CocoaArchive { name, version }
    }
}
