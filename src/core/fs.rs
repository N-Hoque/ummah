use crate::types::{AdhanError, AdhanResult};

use serde::Serialize;

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub fn open_file<P: AsRef<Path>>(path: P) -> AdhanResult<File> {
    File::open(path).map_err(AdhanError::IO)
}

pub fn write_file<P: AsRef<Path>>(dir: P, file: P, html_data: String) -> AdhanResult<()> {
    create_dir(&dir)?;

    let mut file = File::create(dir.as_ref().join(file)).map_err(AdhanError::IO)?;
    write!(&mut file, "{}", html_data).map_err(AdhanError::IO)
}

pub fn write_serialized_file<P: AsRef<Path>, T: Serialize>(
    dir: P,
    file: P,
    data: &T,
) -> AdhanResult<()> {
    if std::fs::read_dir(&dir).is_err() {
        std::fs::create_dir_all(&dir).map_err(AdhanError::IO)?;
    }

    let mut file = File::create(dir.as_ref().join(file)).map_err(AdhanError::IO)?;
    serde_yaml::to_writer(&mut file, data).map_err(AdhanError::Serde)
}

pub fn get_month_filepath() -> Option<PathBuf> {
    dirs_next::document_dir().map(|dir| dir.join("adhan"))
}

pub fn get_cache_filepath() -> Option<PathBuf> {
    dirs_next::cache_dir().map(|dir| dir.join("adhan"))
}

fn create_dir<P: AsRef<Path>>(dir: P) -> AdhanResult<()> {
    if std::fs::read_dir(&dir).is_err() {
        std::fs::create_dir_all(dir).map_err(AdhanError::IO)?;
    };
    Ok(())
}
