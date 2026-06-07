use crate::object::{NsFont, NsObject};
use std::collections::HashMap;

pub(crate) type KvStore = HashMap<Key, Value>;
type ObjectId = usize;

#[derive(PartialEq)]
pub(crate) enum Value {
    Integer(i32),
    Real(f64),
    String(String),
    Ref(ObjectId),
    Array(Vec<Value>),
    Dictionary(KvStore),
}

#[derive(Eq, Hash, PartialEq)]
pub(crate) enum Key {
    NsName,
    NsSize,
    NsfFlags,
    Classes,
    Class,
    Objects,
}

impl From<Key> for String {
    fn from(value: Key) -> Self {
        match value {
            Key::NsName => String::from("NSName"),
            Key::NsSize => String::from("NSSize"),
            Key::NsfFlags => String::from("NSfFlags"),
            Key::Classes => String::from("$classes"),
            Key::Class => String::from("$class"),
            Key::Objects => String::from("$objects"),
        }
    }
}

pub(crate) trait CocoaSerializer<T: NsObject> {
    fn serialize(&self, object: T) -> KvStore {
        let mut store = KvStore::new();
        let objects = self.construct_object_data(&object);
        store.insert(Key::Objects, Value::Array(objects));
        todo!()
    }

    fn construct_object_data(&self, object: &T) -> Vec<Value>;
}

pub(crate) struct NsFontSerializer;
impl CocoaSerializer<NsFont> for NsFontSerializer {
    fn construct_object_data(&self, object: &NsFont) -> Vec<Value> {
        let mut store = KvStore::new();
        let mut objects = Vec::new();

        let class_metadata = construct_class_metadata::<NsFont>();
        store.insert(Key::Class, intern_value(class_metadata, &mut objects));
        store.insert(Key::NsName, intern_value(Value::String(object.name.clone()), &mut objects));
        store.insert(Key::NsSize, Value::Real(object.size as f64));
        store.insert(Key::NsfFlags, Value::Integer(object.flags as i32));

        objects.push(Value::Dictionary(store));
        objects
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
    classes_store.insert(Key::Class, class_value);

    Value::Dictionary(classes_store)
}

fn intern_value(value: Value, objects: &mut Vec<Value>) -> Value {
    objects.iter().position(|v| v == &value).map_or_else(
        || {
            objects.push(value);
            Value::Ref(objects.len() - 1)
        },
        |index| Value::Ref(index),
    )
}
