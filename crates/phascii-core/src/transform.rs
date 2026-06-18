use crate::{AsciiConfig, AsciiImage, PhasciiError, ToneConfig};

pub(super) fn jpg_bytes_to_ascii(
    bytes: &[u8],
    config: &AsciiConfig,
) -> Result<AsciiImage, PhasciiError> {
    let image =
        image::load_from_memory(bytes).map_err(|err| PhasciiError::Decode(err.to_string()))?;
    let rgb = image.to_rgb8();
    let (source_width, source_height) = rgb.dimensions();

    if source_width == 0 || source_height == 0 {
        return Err(PhasciiError::Decode(
            "decoded image has empty dimensions".to_string(),
        ));
    }

    let width_chars = config.width.max(1);
    let height_chars = compute_height_chars(
        source_width,
        source_height,
        width_chars,
        config.aspect_correction,
    );

    let mut values =
        Vec::with_capacity((width_chars as usize).saturating_mul(height_chars as usize));

    for row in 0..height_chars {
        let y_range = cell_range(row, height_chars, source_height);
        for col in 0..width_chars {
            let x_range = cell_range(col, width_chars, source_width);
            values.push(average_luma(&rgb, x_range, y_range));
        }
    }

    let values = if config.normalize {
        normalize_values(&values)
    } else {
        values
    };
    let values = apply_tone_values(&values, config.tone);

    let ramp_chars: Vec<char> = config.ramp.as_str().chars().collect();
    let mut text =
        String::with_capacity((width_chars as usize + 1).saturating_mul(height_chars as usize));

    for row in 0..height_chars as usize {
        for col in 0..width_chars as usize {
            let brightness = values[row * width_chars as usize + col];
            text.push(ramp_char(&ramp_chars, brightness, config.invert));
        }
        text.push('\n');
    }

    Ok(AsciiImage {
        width_chars,
        height_chars,
        text,
    })
}

fn compute_height_chars(
    source_width: u32,
    source_height: u32,
    width_chars: u32,
    aspect_correction: f32,
) -> u32 {
    if source_width == 0 {
        return 1;
    }

    let height =
        ((source_height as f32 / source_width as f32) * width_chars as f32 * aspect_correction)
            .round() as u32;

    height.max(1)
}

fn cell_range(index: u32, output_count: u32, source_count: u32) -> (u32, u32) {
    let start = (u64::from(index) * u64::from(source_count) / u64::from(output_count)) as u32;
    let mut end = (((u64::from(index) + 1) * u64::from(source_count))
        .div_ceil(u64::from(output_count))) as u32;

    if end <= start {
        end = start.saturating_add(1);
    }

    (start.min(source_count), end.min(source_count))
}

fn average_luma(image: &image::RgbImage, x_range: (u32, u32), y_range: (u32, u32)) -> u8 {
    let (x0, x1) = x_range;
    let (y0, y1) = y_range;
    let mut sum = 0u64;
    let mut count = 0u64;

    for y in y0..y1 {
        for x in x0..x1 {
            let pixel = image.get_pixel(x, y).0;
            sum += u64::from(rgb_to_luma(pixel[0], pixel[1], pixel[2]));
            count += 1;
        }
    }

    if count == 0 {
        return 0;
    }

    (sum / count) as u8
}

fn rgb_to_luma(red: u8, green: u8, blue: u8) -> u32 {
    (2126 * u32::from(red) + 7152 * u32::from(green) + 722 * u32::from(blue) + 5000) / 10000
}

fn normalize_values(values: &[u8]) -> Vec<u8> {
    let mut min = u8::MAX;
    let mut max = u8::MIN;

    for &value in values {
        min = min.min(value);
        max = max.max(value);
    }

    if max.saturating_sub(min) < 16 {
        return values.to_vec();
    }

    let range = u32::from(max - min);
    values
        .iter()
        .map(|&value| ((u32::from(value - min) * 255) / range) as u8)
        .collect()
}

fn apply_tone_values(values: &[u8], tone: ToneConfig) -> Vec<u8> {
    values
        .iter()
        .map(|&value| (tone_unit_value(value as f32 / 255.0, tone) * 255.0).round() as u8)
        .collect()
}

fn tone_unit_value(value: f32, tone: ToneConfig) -> f32 {
    let contrast = ((value - 0.5) * tone.contrast + 0.5).clamp(0.0, 1.0);
    contrast.powf(tone.gamma).clamp(0.0, 1.0)
}

fn ramp_char(ramp: &[char], brightness: u8, invert: bool) -> char {
    if ramp.is_empty() {
        return ' ';
    }

    let value = if invert { 255 - brightness } else { brightness };
    let max_index = ramp.len().saturating_sub(1) as u32;
    let index = (u32::from(value) * max_index) / 255;
    ramp[index as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsciiRamp;

    #[test]
    fn height_calculation_matches_spec() {
        assert_eq!(compute_height_chars(4, 2, 2, 1.0), 1);
        assert_eq!(compute_height_chars(2, 4, 3, 1.0), 6);
    }

    #[test]
    fn ramp_mapping_uses_dark_to_light_direction() {
        let ramp = AsciiRamp::default();
        let ramp_chars: Vec<char> = ramp.as_str().chars().collect();
        assert_eq!(ramp_char(&ramp_chars, 0, false), '@');
        assert_eq!(ramp_char(&ramp_chars, 255, false), ' ');
    }

    #[test]
    fn generated_lines_have_expected_width_and_newlines() {
        let image = image::RgbImage::from_fn(2, 2, |x, y| {
            if x == 0 && y == 0 {
                image::Rgb([0, 0, 0])
            } else {
                image::Rgb([255, 255, 255])
            }
        });

        let mut bytes = Vec::new();
        {
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut bytes, 100);
            encoder
                .encode_image(&image)
                .expect("fixture encoding should succeed");
        }

        let config = AsciiConfig {
            width: 2,
            aspect_correction: 1.0,
            invert: false,
            normalize: false,
            tone: ToneConfig::default(),
            ramp: AsciiRamp::default(),
        };

        let ascii = jpg_bytes_to_ascii(&bytes, &config).expect("jpeg should decode");
        assert_eq!(ascii.width_chars, 2);
        assert_eq!(ascii.height_chars, 2);
        assert!(ascii.text.ends_with('\n'));
        let lines: Vec<&str> = ascii.text.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines.iter().all(|line| line.chars().count() == 2));
    }

    #[test]
    fn tone_mapping_clamps_to_valid_range() {
        let tone = ToneConfig {
            contrast: 5.0,
            gamma: 0.2,
        };

        let dark = tone_unit_value(0.0, tone);
        let mid = tone_unit_value(0.5, tone);
        let light = tone_unit_value(1.0, tone);

        assert!((0.0..=1.0).contains(&dark));
        assert!((0.0..=1.0).contains(&mid));
        assert!((0.0..=1.0).contains(&light));
    }
}
