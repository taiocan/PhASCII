use crate::{AsciiImage, PhasciiError};

/// PNG preview configuration for rendering ASCII text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderConfig {
    pub cell_width: u32,
    pub cell_height: u32,
    pub margin: u32,
    pub dark_on_light: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            cell_width: 8,
            cell_height: 12,
            margin: 8,
            dark_on_light: true,
        }
    }
}

impl RenderConfig {
    /// Validate the render configuration.
    pub fn validate(&self) -> Result<(), PhasciiError> {
        if self.cell_width == 0 {
            return Err(PhasciiError::InvalidConfig(
                "cell_width must be greater than 0".to_string(),
            ));
        }

        if self.cell_height == 0 {
            return Err(PhasciiError::InvalidConfig(
                "cell_height must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

/// Render ASCII text into PNG bytes for a preview artifact.
pub fn render_ascii_to_png_bytes(
    ascii: &AsciiImage,
    config: &RenderConfig,
) -> Result<Vec<u8>, PhasciiError> {
    config.validate()?;
    validate_ascii_image(ascii)?;

    #[cfg(feature = "png-preview")]
    {
        render_ascii_to_png_bytes_impl(ascii, config)
    }

    #[cfg(not(feature = "png-preview"))]
    {
        let _ = (ascii, config);
        Err(PhasciiError::Unsupported(
            "PNG preview support is disabled".to_string(),
        ))
    }
}

fn validate_ascii_image(ascii: &AsciiImage) -> Result<(), PhasciiError> {
    if ascii.width_chars == 0 {
        return Err(PhasciiError::Render(
            "ASCII image width must be greater than 0".to_string(),
        ));
    }

    if ascii.height_chars == 0 {
        return Err(PhasciiError::Render(
            "ASCII image height must be greater than 0".to_string(),
        ));
    }

    if !ascii.text.ends_with('\n') {
        return Err(PhasciiError::Render(
            "ASCII image text must end with a trailing newline".to_string(),
        ));
    }

    let lines: Vec<&str> = ascii.text.split_terminator('\n').collect();
    if lines.len() != ascii.height_chars as usize {
        return Err(PhasciiError::Render(format!(
            "ASCII image height mismatch: expected {} lines, found {}",
            ascii.height_chars,
            lines.len()
        )));
    }

    for (index, line) in lines.iter().enumerate() {
        let width = line.chars().count() as u32;
        if width != ascii.width_chars {
            return Err(PhasciiError::Render(format!(
                "ASCII image width mismatch on line {}: expected {}, found {}",
                index + 1,
                ascii.width_chars,
                width
            )));
        }
    }

    Ok(())
}

#[cfg(feature = "png-preview")]
fn render_ascii_to_png_bytes_impl(
    ascii: &AsciiImage,
    config: &RenderConfig,
) -> Result<Vec<u8>, PhasciiError> {
    use font8x8::{UnicodeFonts, BASIC_FONTS};
    use image::{DynamicImage, ImageOutputFormat, Luma};

    let (width, height) = output_dimensions(ascii, config)?;
    let background = if config.dark_on_light { 255 } else { 0 };
    let foreground = if config.dark_on_light { 0 } else { 255 };

    let mut image =
        image::ImageBuffer::<Luma<u8>, Vec<u8>>::from_pixel(width, height, Luma([background]));

    let lines: Vec<&str> = ascii.text.split_terminator('\n').collect();
    let glyph_width = 8u32;
    let glyph_height = 8u32;
    let x_pad = config.cell_width.saturating_sub(glyph_width) / 2;
    let y_pad = config.cell_height.saturating_sub(glyph_height) / 2;

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let glyph = BASIC_FONTS
                .get(ch)
                .or_else(|| BASIC_FONTS.get('?'))
                .unwrap_or([0; 8]);
            let x_origin = config.margin + (col as u32 * config.cell_width) + x_pad;
            let y_origin = config.margin + (row as u32 * config.cell_height) + y_pad;
            draw_glyph(
                &mut image, x_origin, y_origin, glyph, foreground, background,
            );
        }
    }

    let dynamic = DynamicImage::ImageLuma8(image);
    let mut cursor = std::io::Cursor::new(Vec::new());
    dynamic
        .write_to(&mut cursor, ImageOutputFormat::Png)
        .map_err(|err| PhasciiError::Render(err.to_string()))?;

    Ok(cursor.into_inner())
}

#[cfg(feature = "png-preview")]
fn output_dimensions(
    ascii: &AsciiImage,
    config: &RenderConfig,
) -> Result<(u32, u32), PhasciiError> {
    let width = config
        .margin
        .checked_mul(2)
        .and_then(|value| value.checked_add(ascii.width_chars.checked_mul(config.cell_width)?))
        .ok_or_else(|| PhasciiError::Render("PNG width overflow".to_string()))?;

    let height = config
        .margin
        .checked_mul(2)
        .and_then(|value| value.checked_add(ascii.height_chars.checked_mul(config.cell_height)?))
        .ok_or_else(|| PhasciiError::Render("PNG height overflow".to_string()))?;

    Ok((width, height))
}

#[cfg(feature = "png-preview")]
fn draw_glyph(
    image: &mut image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    x_origin: u32,
    y_origin: u32,
    glyph: [u8; 8],
    foreground: u8,
    background: u8,
) {
    for (row_index, row_bits) in glyph.iter().enumerate() {
        for col_index in 0..8u32 {
            let bit = (row_bits >> col_index) & 1;
            if bit == 1 {
                let x = x_origin + col_index;
                let y = y_origin + row_index as u32;
                if x < image.width() && y < image.height() {
                    image.put_pixel(x, y, image::Luma([foreground]));
                }
            }
        }
    }

    let _ = background;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_dimensions_match_formula() {
        let ascii = AsciiImage {
            width_chars: 2,
            height_chars: 2,
            text: "++\n++\n".to_string(),
        };
        let config = RenderConfig::default();
        let bytes = render_ascii_to_png_bytes(&ascii, &config).expect("render should succeed");

        let image = image::load_from_memory(&bytes).expect("png should decode");
        assert_eq!(image.width(), 2 * 8 + 2 * 8);
        assert_eq!(image.height(), 2 * 8 + 2 * 12);
    }

    #[test]
    fn empty_ascii_image_is_rejected() {
        let ascii = AsciiImage {
            width_chars: 0,
            height_chars: 0,
            text: String::new(),
        };
        let err = render_ascii_to_png_bytes(&ascii, &RenderConfig::default())
            .expect_err("empty ascii image must fail");
        assert!(matches!(err, PhasciiError::Render(_)));
    }

    #[test]
    fn inconsistent_ascii_image_is_rejected() {
        let ascii = AsciiImage {
            width_chars: 2,
            height_chars: 2,
            text: "++\n".to_string(),
        };
        let err = render_ascii_to_png_bytes(&ascii, &RenderConfig::default())
            .expect_err("inconsistent ascii image must fail");
        assert!(matches!(err, PhasciiError::Render(_)));
    }
}
