use crate::object::{NsColor, NsFont, NsObject};
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

    NsRgb,
    NsLinearExposure,
    NsComponents,
    NsColorSpace,
}

#[derive(PartialEq)]
pub enum Value {
    Integer(i32),
    Real(f64),
    String(String),
    Data(Vec<u8>),
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

            Key::NsRgb => String::from("NSRGB"),
            Key::NsLinearExposure => String::from("NSLinearExposure"),
            Key::NsComponents => String::from("NSComponents"),
            Key::NsColorSpace => String::from("NSColorSpace"),
        }
    }
}

pub trait CocoaSerializer<T: NsObject> {
    fn serialize(&self, object: T) -> KvStore {
        let mut store = KvStore::new();

        let mut objects = Vec::new();
        objects.push(Value::String(String::from("$null")));

        populate_store(self, &object, &mut objects);
        let root_id = (objects.len() - 1) as u64;

        store.insert(Key::Objects, Value::Array(objects));

        let mut top_store = KvStore::new();
        top_store.insert(Key::Root, Value::Ref(root_id));

        store.insert(Key::Top, Value::Dictionary(top_store));

        store
    }

    fn construct_object_data(&self, object: &T, objects: &mut Vec<Value>, root_store: &mut KvStore);
}

pub struct NsFontSerializer;
impl CocoaSerializer<NsFont> for NsFontSerializer {
    fn construct_object_data(
        &self,
        object: &NsFont,
        objects: &mut Vec<Value>,
        root_store: &mut KvStore,
    ) {
        root_store.insert(
            Key::NsName,
            intern_value(Value::String(object.name.clone()), objects),
        );
        root_store.insert(Key::NsSize, Value::Real(object.size as f64));
        root_store.insert(Key::NsfFlags, Value::Integer(object.flags as i32));
    }
}

pub struct NsColorSerializer;
impl CocoaSerializer<NsColor> for NsColorSerializer {
    fn construct_object_data(
        &self,
        object: &NsColor,
        _objects: &mut Vec<Value>,
        root_store: &mut KvStore,
    ) {
        let mut color_string = String::from(object);
        color_string.push('\0');
        root_store.insert(Key::NsRgb, Value::Data(color_string.into_bytes()));

        root_store.insert(Key::NsColorSpace, Value::Integer(1));
        root_store.insert(Key::NsLinearExposure, Value::Data(b"0".to_vec()));
        root_store.insert(Key::NsComponents, Value::Data(String::from(object).into_bytes()));
    }
}

fn populate_store<S, T>(serializer: &S, object: &T, objects: &mut Vec<Value>)
where
    S: CocoaSerializer<T> + ?Sized,
    T: NsObject,
{
    let mut root_store = KvStore::new();
    let class_metadata = construct_class_metadata::<T>();
    root_store.insert(Key::Class, intern_value(class_metadata, objects));

    serializer.construct_object_data(object, objects, &mut root_store);
    objects.push(Value::Dictionary(root_store));
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
