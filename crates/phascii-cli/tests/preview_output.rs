use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn cli_writes_png_preview_next_to_text_output() {
    let bin = env::var("CARGO_BIN_EXE_phascii-cli")
        .or_else(|_| env::var("CARGO_BIN_EXE_phascii_cli"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| fallback_binary_path());

    let temp_dir = unique_temp_dir();
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");
    let input_path = temp_dir.join("fixture.jpg");
    fs::write(
        &input_path,
        include_bytes!("../../../test-assets/input/2x2_bw.jpg"),
    )
    .expect("fixture should be written");

    let output = Command::new(bin)
        .current_dir(&temp_dir)
        .arg(&input_path)
        .output()
        .expect("cli should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output_dir = temp_dir.join("output");
    let mut txt = None;
    let mut png = None;
    for entry in fs::read_dir(&output_dir).expect("output dir should exist") {
        let path = entry.expect("entry should be readable").path();
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("txt") => txt = Some(path),
            Some("png") => png = Some(path),
            _ => {}
        }
    }

    let txt = txt.expect("text output should exist");
    let png = png.expect("png output should exist");
    let txt_stem = txt
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("txt stem");
    let png_stem = png
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("png stem");
    assert_eq!(txt_stem, png_stem);

    let png_bytes = fs::read(&png).expect("png should be readable");
    assert!(
        png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
        "png signature should be present"
    );
}

#[test]
fn cli_accepts_contrast_and_gamma_flags() {
    let (status, stderr, output_dir) =
        run_cli(&["--width", "2", "--contrast", "1.25", "--gamma", "0.85"]);
    assert!(status.success(), "stderr: {stderr}");

    let preview_path = find_output_file(&output_dir, "png");
    let text_path = find_output_file(&output_dir, "txt");
    assert_same_stem(&text_path, &preview_path);
}

#[test]
fn cli_rejects_invalid_contrast_values_clearly() {
    let (status, stderr, _) = run_cli(&["--width", "2", "--contrast", "0.05"]);
    assert!(!status.success(), "stderr: {stderr}");
    assert!(stderr.contains("contrast"));
}

#[test]
fn cli_rejects_invalid_gamma_values_clearly() {
    let (status, stderr, _) = run_cli(&["--width", "2", "--gamma", "0.1"]);
    assert!(!status.success(), "stderr: {stderr}");
    assert!(stderr.contains("gamma"));
}

fn unique_temp_dir() -> PathBuf {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    let seq = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    env::temp_dir().join(format!("phascii-m2-{}-{stamp}-{seq}", std::process::id()))
}

fn fallback_binary_path() -> PathBuf {
    let exe = env::current_exe().expect("current test executable should be available");
    let debug_dir = exe
        .parent()
        .and_then(|path| path.parent())
        .expect("test binary should live under target/debug/deps");
    debug_dir.join("phascii-cli")
}

fn run_cli(args: &[&str]) -> (std::process::ExitStatus, String, PathBuf) {
    let bin = env::var("CARGO_BIN_EXE_phascii-cli")
        .or_else(|_| env::var("CARGO_BIN_EXE_phascii_cli"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| fallback_binary_path());

    let temp_dir = unique_temp_dir();
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");
    let input_path = temp_dir.join("fixture.jpg");
    fs::write(
        &input_path,
        include_bytes!("../../../test-assets/input/2x2_bw.jpg"),
    )
    .expect("fixture should be written");

    let output = Command::new(bin)
        .current_dir(&temp_dir)
        .arg(&input_path)
        .args(args)
        .output()
        .expect("cli should run");

    (
        output.status,
        String::from_utf8_lossy(&output.stderr).into_owned(),
        temp_dir.join("output"),
    )
}

fn find_output_file(output_dir: &Path, ext: &str) -> PathBuf {
    for entry in fs::read_dir(output_dir).expect("output dir should exist") {
        let path = entry.expect("entry should be readable").path();
        if path.extension().and_then(|value| value.to_str()) == Some(ext) {
            return path;
        }
    }

    panic!("expected {ext} output");
}

fn assert_same_stem(left: &Path, right: &Path) {
    let left_stem = left
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("left stem");
    let right_stem = right
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("right stem");
    assert_eq!(left_stem, right_stem);
}
