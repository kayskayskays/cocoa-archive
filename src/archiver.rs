use crate::object::NsObject;
use crate::serializer::CocoaSerializer;

enum NsArchiver {
    NsKeyedArchiver,
}

impl NsArchiver {
    fn ns_keyed_archive<T, U>(&self, object: T, serializer: U) -> CocoaArchive
    where
        T: NsObject,
        U: CocoaSerializer<T>,
    {
        let store = serializer.serialize(object);
        todo!()
    }
}

impl Archiver for NsArchiver {
    fn archive<T, U>(&self, object: T, serializer: U) -> CocoaArchive
    where
        T: NsObject,
        U: CocoaSerializer<T>,
    {
        match self {
            NsArchiver::NsKeyedArchiver => self.ns_keyed_archive(object, serializer),
        }
    }
}

trait Archiver {
    fn archive<T, U>(&self, object: T, serializer: U) -> CocoaArchive
    where
        T: NsObject,
        U: CocoaSerializer<T>;
}

struct ArchiverOptions {
    version: u32,
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
