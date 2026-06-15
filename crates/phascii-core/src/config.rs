use crate::error::PhasciiError;

/// Output configuration for ASCII conversion.
#[derive(Debug, Clone, PartialEq)]
pub struct AsciiConfig {
    pub width: u32,
    pub aspect_correction: f32,
    pub invert: bool,
    pub normalize: bool,
    pub ramp: AsciiRamp,
}

impl Default for AsciiConfig {
    fn default() -> Self {
        Self {
            width: 120,
            aspect_correction: 0.5,
            invert: false,
            normalize: true,
            ramp: AsciiRamp::default(),
        }
    }
}

impl AsciiConfig {
    /// Validate the configuration for basic bootstrap requirements.
    pub fn validate(&self) -> Result<(), PhasciiError> {
        if self.width == 0 {
            return Err(PhasciiError::InvalidConfig(
                "width must be greater than 0".to_string(),
            ));
        }

        if self.width > 500 {
            return Err(PhasciiError::InvalidConfig(
                "width must not exceed 500 for now".to_string(),
            ));
        }

        if self.aspect_correction <= 0.0 {
            return Err(PhasciiError::InvalidConfig(
                "aspect_correction must be greater than 0.0".to_string(),
            ));
        }

        if self.ramp.len_chars() < 2 {
            return Err(PhasciiError::InvalidConfig(
                "ramp must contain at least 2 characters".to_string(),
            ));
        }

        Ok(())
    }
}

/// Ordered grayscale ramp from darkest to lightest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiRamp(String);

impl AsciiRamp {
    /// Create a ramp from any string-like input.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the ramp as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Count Unicode scalar values in the ramp.
    pub fn len_chars(&self) -> usize {
        self.0.chars().count()
    }
}

impl Default for AsciiRamp {
    fn default() -> Self {
        Self::new("@%#*+=-:. ")
    }
}

impl From<&str> for AsciiRamp {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

/// Placeholder transform mode enum for future work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformMode {
    Stylized,
    Faithful,
    DitheredExperimental,
}

impl Default for TransformMode {
    fn default() -> Self {
        Self::Stylized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_expected_width() {
        assert_eq!(AsciiConfig::default().width, 120);
    }

    #[test]
    fn default_ramp_matches_spec() {
        assert_eq!(AsciiConfig::default().ramp.as_str(), "@%#*+=-:. ");
    }

    #[test]
    fn default_config_validates() {
        assert!(AsciiConfig::default().validate().is_ok());
    }

    #[test]
    fn width_zero_fails_validation() {
        let mut config = AsciiConfig::default();
        config.width = 0;

        let err = config.validate().expect_err("width zero must be invalid");
        assert!(matches!(err, PhasciiError::InvalidConfig(_)));
    }

    #[test]
    fn width_over_limit_fails_validation() {
        let mut config = AsciiConfig::default();
        config.width = 501;

        let err = config
            .validate()
            .expect_err("width over limit must be invalid");
        assert!(matches!(err, PhasciiError::InvalidConfig(_)));
    }

    #[test]
    fn empty_ramp_fails_validation() {
        let mut config = AsciiConfig::default();
        config.ramp = AsciiRamp::new("");

        let err = config.validate().expect_err("empty ramp must be invalid");
        assert!(matches!(err, PhasciiError::InvalidConfig(_)));
    }

    #[test]
    fn one_character_ramp_fails_validation() {
        let mut config = AsciiConfig::default();
        config.ramp = AsciiRamp::new("@");

        let err = config
            .validate()
            .expect_err("single-character ramp must be invalid");
        assert!(matches!(err, PhasciiError::InvalidConfig(_)));
    }
}
