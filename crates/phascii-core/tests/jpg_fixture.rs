use phascii_core::{jpg_bytes_to_ascii, AsciiConfig};

#[test]
fn tiny_jpg_fixture_decodes_to_ascii_text() {
    let bytes = include_bytes!("../../../test-assets/input/2x2_bw.jpg");
    let config = AsciiConfig {
        width: 2,
        aspect_correction: 1.0,
        invert: false,
        normalize: false,
        ..AsciiConfig::default()
    };

    let ascii = jpg_bytes_to_ascii(bytes, &config).expect("fixture should decode");

    assert_eq!(ascii.width_chars, 2);
    assert_eq!(ascii.height_chars, 2);
    assert!(ascii.text.ends_with('\n'));

    let lines: Vec<&str> = ascii.text.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.iter().all(|line| line.chars().count() == 2));
}
