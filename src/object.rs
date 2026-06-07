pub(crate) trait NsObject {
    fn class() -> &'static str;
    fn classes() -> Vec<&'static str>;
}

pub(crate) struct NsFont {
    pub(crate) name: String,
    pub(crate) size: f32,
    pub(crate) flags: u32,
}

impl NsFont {
    const DEFAULT_FLAGS: u32 = 16;

    fn new(name: String, size: f32) -> Self {
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

    fn classes() -> Vec<&'static str> {
        vec!["NSFont", "NSObject"]
    }
}

pub(crate) struct NsColor {}
