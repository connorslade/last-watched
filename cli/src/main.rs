use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;
use sidecar::{open_or_create_sidecar, open_sidecar, Sidecar};

mod sidecar;
mod winapi;

#[derive(Parser)]
pub enum Cli {
    /// Mark a video file as watched
    Watched {
        /// File to mark. Must be a video file.
        file: PathBuf,
    },
    /// Mark a video file as unwatched
    Unwatched {
        /// File to mark. Must be a video file.
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args {
        Cli::Watched { file } => {
            let Some(file_name) = file.file_name() else {
                bail!("Provided path is not a file");
            };

            let sidecar = open_or_create_sidecar(&file)?;
            let mut sidecar = Sidecar::new(sidecar)?;
            sidecar.add(&file_name.to_string_lossy())?;
        }
        Cli::Unwatched { file } => {
            let Some(file_name) = file.file_name() else {
                bail!("Provided path is not a file");
            };

            let Some(sidecar) = open_sidecar(&file) else {
                return Ok(());
            };

            let mut sidecar = Sidecar::new(sidecar?)?;
            sidecar.remove(&file_name.to_string_lossy())?;
        }
    }

    Ok(())
}
