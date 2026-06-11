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

pub(crate) type CocoaKeyValueStore = HashMap<Key, Value>;

pub(crate) trait CocoaSerializer<T: NsObject> {
    /// Serializes an [`NsObject`] object into a [`CocoaKeyValueStore`].
    ///
    /// # Implementation Details
    ///
    /// Generally speaking, and as a simplification, Cocoa archives in XML plist format look like the
    /// following:
    ///
    /// ```xml
    /// <plist>
    /// <dict>
    ///     <key>$objects</key> // The objects in the archive.
    ///     <array>
    ///         <string>$null</string> // This will always be the zeroth object.
    ///         <dict> // A representation of the object's class hierarchy.
    ///             <key>$classname</key>
    ///             <string>[CLASSNAME]</string>
    ///             <key>$classes</key>
    ///             <array>
    ///                 // Classnames in the class hierarchy...
    ///             </array>
    ///         </dict>
    ///         <dict> // This is the "root store", in our model.
    ///             <key>$class</key> // This key will always be present.
    ///             <uid>1<uid> // These UIDs are indexes into the `$objects` array.
    ///             <key>
    ///                 // Implementation specific object data...
    ///             </key>
    ///         </dict>
    ///         <dict>
    ///             // Other objects, also referencable by UID...
    ///         </dict>
    ///     </array>
    ///     <key>$top</key>
    ///     <dict>
    ///         <key>root</key>
    ///         <uid>2</uid> // This is the UID of the "root store" object.
    ///     </dict>
    /// </dict>
    /// </plist>
    /// ```
    ///
    /// This default function handles the creation of the `$objects` array, the
    /// insertion of the `$null` object into the `$objects` array, the
    /// construction of the class hierarchy object, and the construction of the
    /// `$top` dictionary.
    ///
    /// Implementations of this trait are responsible for populating the
    /// remainder of the `$objects` array
    /// (see [`CocoaSerializer::construct_object_data`], [`populate_objects`]), which includes
    /// populating not only the "root store", but also any other objects
    /// referenced by it.
    ///
    /// The returned [`CocoaKeyValueStore`] is representative of the top-level
    /// `<dict>` element. In fact, each `<dict>` element in the description
    /// above is represented by a [`CocoaKeyValueStore`] in our model.
    fn serialize(&self, object: T) -> CocoaKeyValueStore {
        let mut store = HashMap::new();
        let mut objects = Vec::new();

        // The first object in the archive is always the `$null` object.
        objects.push(Value::String(String::from("$null")));

        // Populate the `$objects` array.
        populate_objects(self, &object, &mut objects);
        let object_count = objects.len();
        store.insert(Key::Objects, Value::Array(objects));

        // Construct the `$top` dictionary.
        construct_and_insert_top_store((object_count - 1) as Uid, &mut store);
        store
    }

    /// Constructs the `$objects` array of a Cocoa plist archive.
    ///
    /// # Implementation Requirements
    ///
    /// Implementors are primarily required to populate the provided
    /// `root_store` with the data of the `object` being serialized.
    ///
    /// While doing so, it may be the case that serialization will require
    /// references to other objects in the archive. In such a case, those
    /// objects should be inserted into the `objects` vector. The
    /// `root_store` must be updated with [`Value::Ref`] wrappers around the
    /// [`Uid`] of those objects where required - see [`intern_value`] for a
    /// utility enabling this.
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

/// Populates the `$objects` array of a Cocoa plist archive.
///
/// # Invariants
///
/// The primary invariant we seek to uphold here is that the "root" object data is always the last
/// object within the `objects` vector. This is so upstream consumers always have a simple means
/// of referencing the root object.
fn populate_objects<S, T>(serializer: &S, object: &T, objects: &mut Vec<Value>)
where
    S: CocoaSerializer<T> + ?Sized,
    T: NsObject,
{
    let mut root_store = HashMap::new();
    let class_metadata = construct_class_metadata::<T>();
    root_store.insert(Key::Class, intern_value(class_metadata, objects));

    serializer.construct_object_data(object, objects, &mut root_store);
    objects.push(Value::Dictionary(root_store));
}

fn construct_and_insert_top_store(root_store_uid: Uid, store: &mut CocoaKeyValueStore) {
    let mut top_store = HashMap::new();
    top_store.insert(Key::Root, Value::Ref(root_store_uid));

    store.insert(Key::Top, Value::Dictionary(top_store));
}

fn construct_class_metadata<T>() -> Value
where
    T: NsObject,
{
    let classes_value = Value::Array(
        T::classes()
            .iter()
            .map(|class| Value::String(class.to_string()))
            .collect(),
    );

    let class_value = Value::String(T::class().to_string());

    let mut classes_store = HashMap::new();
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
