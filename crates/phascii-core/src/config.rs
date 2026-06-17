use crate::error::PhasciiError;

/// Output configuration for ASCII conversion.
#[derive(Debug, Clone, PartialEq)]
pub struct AsciiConfig {
    pub width: u32,
    pub aspect_correction: f32,
    pub invert: bool,
    pub normalize: bool,
    pub tone: ToneConfig,
    pub ramp: AsciiRamp,
}

/// Optional tone controls applied before ramp mapping.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToneConfig {
    pub contrast: f32,
    pub gamma: f32,
}

impl Default for AsciiConfig {
    fn default() -> Self {
        Self {
            width: 120,
            aspect_correction: 0.5,
            invert: false,
            normalize: true,
            tone: ToneConfig::default(),
            ramp: AsciiRamp::default(),
        }
    }
}

impl Default for ToneConfig {
    fn default() -> Self {
        Self {
            contrast: 1.0,
            gamma: 1.0,
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

        self.tone.validate()?;

        Ok(())
    }
}

impl ToneConfig {
    /// Validate the optional tone controls.
    pub fn validate(&self) -> Result<(), PhasciiError> {
        if !self.contrast.is_finite() {
            return Err(PhasciiError::InvalidConfig(
                "contrast must be finite".to_string(),
            ));
        }

        if !(0.1..=5.0).contains(&self.contrast) {
            return Err(PhasciiError::InvalidConfig(
                "contrast must be between 0.1 and 5.0".to_string(),
            ));
        }

        if !self.gamma.is_finite() {
            return Err(PhasciiError::InvalidConfig(
                "gamma must be finite".to_string(),
            ));
        }

        if !(0.2..=5.0).contains(&self.gamma) {
            return Err(PhasciiError::InvalidConfig(
                "gamma must be between 0.2 and 5.0".to_string(),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransformMode {
    #[default]
    Stylized,
    Faithful,
    DitheredExperimental,
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
    fn tone_config_defaults_match_spec() {
        let tone = ToneConfig::default();
        assert_eq!(tone.contrast, 1.0);
        assert_eq!(tone.gamma, 1.0);
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

    #[test]
    fn invalid_contrast_fails_validation() {
        let mut tone = ToneConfig::default();
        tone.contrast = 0.05;

        let err = tone
            .validate()
            .expect_err("contrast below minimum must be invalid");
        assert!(matches!(err, PhasciiError::InvalidConfig(_)));
    }

    #[test]
    fn invalid_gamma_fails_validation() {
        let mut tone = ToneConfig::default();
        tone.gamma = 0.1;

        let err = tone
            .validate()
            .expect_err("gamma below minimum must be invalid");
        assert!(matches!(err, PhasciiError::InvalidConfig(_)));
    }

    #[test]
    fn non_finite_tone_values_fail_validation() {
        let tone = ToneConfig {
            contrast: f32::NAN,
            gamma: f32::INFINITY,
        };

        let err = tone
            .validate()
            .expect_err("non-finite tone values must be invalid");
        assert!(matches!(err, PhasciiError::InvalidConfig(_)));
    }
}
