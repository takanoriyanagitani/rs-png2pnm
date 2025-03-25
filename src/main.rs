use std::process::ExitCode;

use std::io;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(|e| io::Error::other(format!("env val {key} missing: {e}")))
}

fn png_filename() -> Result<String, io::Error> {
    env_val_by_key("ENV_I_PNG_FILENAME")
}

fn ppm_filename() -> Result<String, io::Error> {
    env_val_by_key("ENV_O_PPM_FILENAME")
}

fn sub() -> Result<(), io::Error> {
    let ipng: String = png_filename()?;
    let oppm: String = ppm_filename()?;

    rs_png2pnm::pngfilename2ppmfilename(&ipng, &oppm)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
