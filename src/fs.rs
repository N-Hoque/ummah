//! Helper module for file IO

use crate::types::{UmmahError, UmmahResult};

use serde::Serialize;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

/// Gets user document directory for core files. Files are stored in "adhan" directory
///
/// The user document differs between OSes.
pub fn get_user_filepath() -> PathBuf {
    dirs_next::document_dir().map_or_else(|| "adhan".into(), |dir| dir.join("adhan"))
}

/// Gets cache directory for core files. Files are stored in "adhan" directory
///
/// The cache directory differs between OSes.
pub fn get_cache_filepath() -> PathBuf {
    dirs_next::cache_dir().map_or_else(|| "adhan".into(), |dir| dir.join("adhan"))
}

pub(crate) fn open_file<P: AsRef<Path>>(path: P) -> UmmahResult<File> {
    File::open(path).map_err(UmmahError::IO)
}

pub(crate) fn write_serialized_file<P: AsRef<Path>, T: Serialize>(
    dir: P,
    file: P,
    data: &T,
) -> UmmahResult<()> {
    create_dir(&dir)?;

    let path = dir.as_ref().join(file);

    println!("Serializing data to {:?}", path);

    let mut file = File::create(path).map_err(UmmahError::IO)?;
    serde_yaml::to_writer(&mut file, data).map_err(UmmahError::Serde)
}

fn create_dir<P: AsRef<Path>>(dir: P) -> UmmahResult<()> {
    if std::fs::read_dir(&dir).is_err() {
        std::fs::create_dir_all(dir).map_err(UmmahError::IO)?;
    };
    Ok(())
}
