use std::fs;
use std::path::Path;

use fileflow_core::{FileFlowError, Result};

pub fn validate_source_file(src: &Path) -> Result<()> {
    if !src.exists() {
        return Err(FileFlowError::Message(format!(
            "Source does not exist: {}",
            src.display()
        )));
    }

    if !src.is_file() {
        return Err(FileFlowError::Message(format!(
            "Source is not a file: {}",
            src.display()
        )));
    }

    Ok(())
}

pub fn prepare_destination(dst: &Path, overwrite: bool) -> Result<()> {
    if dst.exists() && !overwrite {
        return Err(FileFlowError::Message(format!(
            "Destination exists (use --overwrite): {}",
            dst.display()
        )));
    }

    if let Some(parent) = dst.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(())
}

pub fn remove_destination_if_overwrite(dst: &Path, overwrite: bool) -> Result<()> {
    if dst.exists() && overwrite {
        if dst.is_file() {
            fs::remove_file(dst)?;
        } else {
            return Err(FileFlowError::Message(format!(
                "Destination exists and is not a file: {}",
                dst.display()
            )));
        }
    }

    Ok(())
}