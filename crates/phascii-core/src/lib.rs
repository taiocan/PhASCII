//! Core types and APIs for PhASCII.

pub mod config;
pub mod error;
pub mod metrics;
pub mod output;
pub mod render;

pub use config::{AsciiConfig, AsciiRamp, TransformMode};
pub use error::PhasciiError;
pub use metrics::TransformMetrics;
pub use output::AsciiImage;
pub use render::{render_ascii_to_png_bytes, RenderConfig};

#[cfg(feature = "jpg")]
mod transform;

/// Convert JPG bytes into deterministic ASCII text.
pub fn jpg_bytes_to_ascii(bytes: &[u8], config: &AsciiConfig) -> Result<AsciiImage, PhasciiError> {
    config.validate()?;

    if bytes.is_empty() {
        return Err(PhasciiError::Decode("input is empty".to_string()));
    }

    #[cfg(feature = "jpg")]
    {
        transform::jpg_bytes_to_ascii(bytes, config)
    }

    #[cfg(not(feature = "jpg"))]
    {
        let _ = bytes;
        Err(PhasciiError::Unsupported(
            "JPG support is disabled".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_config_fails_before_decode() {
        let mut config = AsciiConfig::default();
        config.width = 0;

        let err = jpg_bytes_to_ascii(b"not a jpg", &config)
            .expect_err("invalid config should fail before decode");
        assert!(matches!(err, PhasciiError::InvalidConfig(_)));
    }

    #[test]
    fn empty_bytes_return_decode_error() {
        let config = AsciiConfig::default();

        let err = jpg_bytes_to_ascii(&[], &config).expect_err("empty bytes must fail");
        assert!(matches!(err, PhasciiError::Decode(_)));
    }
}
