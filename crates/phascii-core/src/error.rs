use core::fmt;

/// Errors returned by the PhASCII core API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhasciiError {
    InvalidConfig(String),
    Decode(String),
    Io(String),
    Render(String),
    Unsupported(String),
}

impl fmt::Display for PhasciiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(message) => write!(f, "invalid config: {message}"),
            Self::Decode(message) => write!(f, "decode error: {message}"),
            Self::Io(message) => write!(f, "io error: {message}"),
            Self::Render(message) => write!(f, "render error: {message}"),
            Self::Unsupported(message) => write!(f, "unsupported: {message}"),
        }
    }
}

impl std::error::Error for PhasciiError {}

impl From<std::io::Error> for PhasciiError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}
