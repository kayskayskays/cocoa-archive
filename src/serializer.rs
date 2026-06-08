use crate::object::{NsFont, NsObject};
use std::collections::HashMap;

pub type KvStore = HashMap<Key, Value>;
type Uid = u64;

#[derive(Eq, Hash, PartialEq)]
pub enum Key {
    NsName,
    NsSize,
    NsfFlags,

    Classes,
    Classname,
    Class,
    Objects,

    Top,
    Root,

    Archiver,
    Version,
}

#[derive(PartialEq)]
pub enum Value {
    Integer(i32),
    Real(f64),
    String(String),
    Ref(Uid),
    Array(Vec<Value>),
    Dictionary(KvStore),
}

impl From<Key> for String {
    fn from(value: Key) -> Self {
        match value {
            Key::NsName => String::from("NSName"),
            Key::NsSize => String::from("NSSize"),
            Key::NsfFlags => String::from("NSfFlags"),

            Key::Classes => String::from("$classes"),
            Key::Classname => String::from("$classname"),
            Key::Class => String::from("$class"),
            Key::Objects => String::from("$objects"),

            Key::Top => String::from("$top"),
            Key::Root => String::from("root"),

            Key::Archiver => String::from("$archiver"),
            Key::Version => String::from("$version"),
        }
    }
}

pub trait CocoaSerializer<T: NsObject> {
    fn serialize(&self, object: T) -> KvStore {
        let mut store = KvStore::new();

        let mut objects = Vec::new();
        objects.push(Value::String(String::from("$null")));

        self.construct_object_data(&object, &mut objects);
        let root_id = (objects.len() - 1) as u64;

        store.insert(Key::Objects, Value::Array(objects));

        let mut top_store = KvStore::new();
        top_store.insert(Key::Root, Value::Ref(root_id));

        store.insert(Key::Top, Value::Dictionary(top_store));

        store
    }

    fn construct_object_data(&self, object: &T, objects: &mut Vec<Value>);
}

pub struct NsFontSerializer;
impl CocoaSerializer<NsFont> for NsFontSerializer {
    fn construct_object_data(&self, object: &NsFont, objects: &mut Vec<Value>) {
        let mut store = KvStore::new();

        let class_metadata = construct_class_metadata::<NsFont>();
        store.insert(Key::Class, intern_value(class_metadata, objects));
        store.insert(
            Key::NsName,
            intern_value(Value::String(object.name.clone()), objects),
        );
        store.insert(Key::NsSize, Value::Real(object.size as f64));
        store.insert(Key::NsfFlags, Value::Integer(object.flags as i32));

        objects.push(Value::Dictionary(store));
    }
}
fn construct_class_metadata<T>() -> Value
where
    T: NsObject,
{
    let classes_value = Value::Array(
        T::classes()
            .into_iter()
            .map(|class| Value::String(class.to_string()))
            .collect(),
    );

    let class_value = Value::String(T::class().to_string());

    let mut classes_store = KvStore::new();
    classes_store.insert(Key::Classes, classes_value);
    classes_store.insert(Key::Classname, class_value);

    Value::Dictionary(classes_store)
}

fn intern_value(value: Value, objects: &mut Vec<Value>) -> Value {
    objects.iter().position(|v| v == &value).map_or_else(
        || {
            objects.push(value);
            Value::Ref((objects.len() - 1) as u64)
        },
        |index| Value::Ref(index as u64),
    )
}
