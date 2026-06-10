use crate::object::{NsColor, NsFont, NsObject};
use std::collections::HashMap;

pub(crate) type Uid = u64;

#[derive(Eq, Hash, PartialEq)]
pub(crate) enum Key {
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
pub(crate) enum Value {
    Integer(i32),
    Real(f64),
    String(String),
    Data(Vec<u8>),
    Ref(Uid),
    Array(Vec<Value>),
    Dictionary(CocoaKeyValueStore),
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

#[derive(PartialEq)]
pub(crate) struct CocoaKeyValueStore {
    store: HashMap<Key, Value>,
}

impl CocoaKeyValueStore {
    pub(crate) fn insert(&mut self, key: Key, value: Value) {
        self.store.insert(key, value);
    }

    pub(crate) fn into_iter(self) -> impl Iterator<Item = (Key, Value)> {
        self.store.into_iter()
    }
}

pub(crate) trait CocoaSerializer<T: NsObject> {
    fn serialize(&self, object: T) -> CocoaKeyValueStore {
        let mut store = HashMap::new();

        let mut objects = Vec::new();

        // The first object in the archive is always the `$null` object.
        objects.push(Value::String(String::from("$null")));
        populate_store(self, &object, &mut objects);
        let root_id = (objects.len() - 1) as u64;

        store.insert(Key::Objects, Value::Array(objects));

        let mut top_store = HashMap::new();
        top_store.insert(Key::Root, Value::Ref(root_id));

        store.insert(
            Key::Top,
            Value::Dictionary(CocoaKeyValueStore { store: top_store }),
        );

        CocoaKeyValueStore { store }
    }

    fn construct_object_data(
        &self,
        object: &T,
        objects: &mut Vec<Value>,
        root_store: &mut CocoaKeyValueStore,
    );
}

pub(crate) struct NsFontSerializer;
impl CocoaSerializer<NsFont> for NsFontSerializer {
    fn construct_object_data(
        &self,
        object: &NsFont,
        objects: &mut Vec<Value>,
        root_store: &mut CocoaKeyValueStore,
    ) {
        root_store.insert(
            Key::NsName,
            intern_value(Value::String(object.name.clone()), objects),
        );
        root_store.insert(Key::NsSize, Value::Real(object.size as f64));

        // This key remains opaque to me, but it seems to default to [`NsFont::DEFAULT_FLAGS`],
        // so that's what we'll use here, for now.
        root_store.insert(Key::NsfFlags, Value::Integer(object.flags as i32));
    }
}

pub(crate) struct NsColorSerializer;
impl CocoaSerializer<NsColor> for NsColorSerializer {
    fn construct_object_data(
        &self,
        object: &NsColor,
        _objects: &mut Vec<Value>,
        root_store: &mut CocoaKeyValueStore,
    ) {
        // Using the bare minimum data required to construct an archived `NSColor` Cocoa object
        // that can be decoded by Apple applications.
        // Turns out, there's no need for a custom color space or an ICC profile - it's sufficient
        // to pass in the color components under the `NSRGB` key, so long as the other keys below
        // are populated with sensible defaults.

        let mut color_string = String::from(object);
        color_string.push('\0');
        root_store.insert(Key::NsRgb, Value::Data(color_string.into_bytes()));
        root_store.insert(Key::NsColorSpace, Value::Integer(1));
        root_store.insert(Key::NsLinearExposure, Value::Data(b"0".to_vec()));
        root_store.insert(
            Key::NsComponents,
            Value::Data(String::from(object).into_bytes()),
        );
    }
}

/// Populates a [`CocoaKeyValueStore`] with the data required to construct a Cocoa object archive.
///
/// # Invariants
/// The primary invariant we seek to uphold here is that the "root" object data is always the last
/// object within the `objects` vector. This is so upstream consumers always have a simple means
/// of referencing the root object.
fn populate_store<S, T>(serializer: &S, object: &T, objects: &mut Vec<Value>)
where
    S: CocoaSerializer<T> + ?Sized,
    T: NsObject,
{
    let mut cocoa_store = CocoaKeyValueStore {
        store: HashMap::new(),
    };
    let class_metadata = construct_class_metadata::<T>();
    cocoa_store.insert(Key::Class, intern_value(class_metadata, objects));

    serializer.construct_object_data(object, objects, &mut cocoa_store);
    objects.push(Value::Dictionary(cocoa_store));
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

    let mut classes_store = CocoaKeyValueStore {
        store: HashMap::new(),
    };
    classes_store.insert(Key::Classes, classes_value);
    classes_store.insert(Key::Classname, class_value);

    Value::Dictionary(classes_store)
}

/// Interns a value into the provided vector of values and returns a [`Value::Ref`] wrapper around
/// it, with a [`Uid`] corresponding to the index of the value in the vector.
///
/// If the value is already present in the vector, the reference points to the existing value.
/// Otherwise, the value is inserted into the vector and the reference is defined accordingly.
///
/// This can be used for constructing more complicated Cocoa objects which store references to other
/// objects in their own serialized form.
fn intern_value(value: Value, objects: &mut Vec<Value>) -> Value {
    objects.iter().position(|v| v == &value).map_or_else(
        || {
            objects.push(value);
            Value::Ref((objects.len() - 1) as u64)
        },
        |index| Value::Ref(index as u64),
    )
}
