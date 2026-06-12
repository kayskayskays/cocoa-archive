pub(crate) trait NsObject {
    fn class() -> &'static str;
    fn classes() -> &'static [&'static str];
}

pub(crate) struct NsFont {
    pub(crate) name: String,
    pub(crate) size: f32,
    pub(crate) flags: u32,
}

impl NsFont {
    const DEFAULT_FLAGS: u32 = 16;

    pub fn new(name: String, size: f32) -> Self {
        Self {
            name,
            size,
            flags: Self::DEFAULT_FLAGS,
        }
    }
}

impl NsObject for NsFont {
    fn class() -> &'static str {
        "NSFont"
    }

    fn classes() -> &'static [&'static str] {
        &["NSFont", "NSObject"]
    }
}

pub(crate) struct NsColor {
    pub(crate) red: f32,
    pub(crate) green: f32,
    pub(crate) blue: f32,
    pub(crate) alpha: f32,
}

impl NsObject for NsColor {
    fn class() -> &'static str {
        "NSColor"
    }

    fn classes() -> &'static [&'static str] {
        &["NSColor", "NSObject"]
    }
}

impl From<&NsColor> for String {
    fn from(value: &NsColor) -> Self {
        format!(
            "{} {} {} {}",
            value.red, value.green, value.blue, value.alpha
        )
    }
}
