use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
    time::SystemTime,
};

use phascii_core::{jpg_bytes_to_ascii, AsciiConfig};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mut input_path: Option<PathBuf> = None;
    let mut width_override: Option<u32> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            }
            "--width" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--width requires a value".to_string())?;
                width_override = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| "--width must be a positive integer".to_string())?,
                );
            }
            value if value.starts_with('-') => {
                return Err(format!("unrecognized argument: {value}"));
            }
            value => {
                if input_path.is_some() {
                    return Err("only one input path is supported".to_string());
                }
                input_path = Some(PathBuf::from(value));
            }
        }
    }

    let input_path =
        input_path.ok_or_else(|| "usage: phascii-cli <input.jpg> [--width N]".to_string())?;
    let input_stem = input_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("input");

    let mut config = AsciiConfig::default();
    if let Some(width) = width_override {
        config.width = width;
    }

    let bytes = fs::read(&input_path)
        .map_err(|err| format!("failed to read {}: {err}", input_path.display()))?;
    let ascii = jpg_bytes_to_ascii(&bytes, &config).map_err(|err| err.to_string())?;

    fs::create_dir_all("output").map_err(|err| format!("failed to create output/: {err}"))?;
    let timestamp = unix_millis();
    let output_path = format!("output/phascii_{input_stem}_{timestamp}.txt");
    fs::write(&output_path, ascii.text.as_bytes())
        .map_err(|err| format!("failed to write {output_path}: {err}"))?;

    let mut stdout = io::stdout().lock();
    stdout
        .write_all(ascii.text.as_bytes())
        .map_err(|err| format!("failed to write stdout: {err}"))?;
    stdout
        .flush()
        .map_err(|err| format!("failed to flush stdout: {err}"))?;

    eprintln!("wrote {output_path}");
    Ok(())
}

fn print_usage() {
    println!("Usage: phascii-cli <input.jpg> [--width N]");
}

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}
