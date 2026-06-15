/// ASCII image output produced by the core library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiImage {
    pub width_chars: u32,
    pub height_chars: u32,
    pub text: String,
}

impl AsciiImage {
    /// Create a new ASCII image payload.
    pub fn new(width_chars: u32, height_chars: u32, text: String) -> Self {
        Self {
            width_chars,
            height_chars,
            text,
        }
    }
}
