use crate::types::{AdhanError, AdhanResult};

use serde::Serialize;

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub fn get_user_filepath() -> PathBuf {
    dirs_next::document_dir().map_or_else(|| "adhan".into(), |dir| dir.join("adhan"))
}

pub fn get_cache_filepath() -> PathBuf {
    dirs_next::cache_dir().map_or_else(|| "adhan".into(), |dir| dir.join("adhan"))
}

pub(crate) fn open_file<P: AsRef<Path>>(path: P) -> AdhanResult<File> {
    File::open(path).map_err(AdhanError::IO)
}

pub(crate) fn write_file<P: AsRef<Path>>(dir: P, file: P, data: &[u8]) -> AdhanResult<()> {
    create_dir(&dir)?;

    let path = dir.as_ref().join(file);

    println!("Writing file to {:?}", path);

    let mut file = File::create(path).map_err(AdhanError::IO)?;

    file.write(data).map(|_| ()).map_err(AdhanError::IO)
}

pub(crate) fn write_serialized_file<P: AsRef<Path>, T: Serialize>(
    dir: P,
    file: P,
    data: &T,
) -> AdhanResult<()> {
    create_dir(&dir)?;

    let path = dir.as_ref().join(file);

    println!("Serializing data to {:?}", path);

    let mut file = File::create(path).map_err(AdhanError::IO)?;
    serde_yaml::to_writer(&mut file, data).map_err(AdhanError::Serde)
}

fn create_dir<P: AsRef<Path>>(dir: P) -> AdhanResult<()> {
    if std::fs::read_dir(&dir).is_err() {
        std::fs::create_dir_all(dir).map_err(AdhanError::IO)?;
    };
    Ok(())
}
