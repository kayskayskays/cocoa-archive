use crate::archiver::ArchiverKeyValueStore;

pub(crate) enum Value {
    Integer(i32),
    Real(f64),
    String(String),
    Array(Vec<Value>),
    Dictionary(Vec<(String, Value)>)
}

pub(crate) trait CocoaSerializer<T> {
    fn serialize(&self, object: T, store: &mut ArchiverKeyValueStore);
}
