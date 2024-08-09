use crate::game::env::Env;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use zip_extensions::*;

#[derive(Debug)]
pub enum Error {
    HttpError(ureq::Error),
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
}

#[derive(Debug)]
pub enum ToJsonError {
    HttpError(ureq::Error),
    IoError(std::io::Error),
}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Self::HttpError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(e: zip::result::ZipError) -> Self {
        Self::ZipError(e)
    }
}

impl From<ureq::Error> for ToJsonError {
    fn from(e: ureq::Error) -> Self {
        Self::HttpError(e)
    }
}

impl From<std::io::Error> for ToJsonError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

pub fn get_json<T: DeserializeOwned>(url: &str) -> Result<T, ToJsonError> {
    Ok(ureq::get(url).call()?.into_json()?)
}

pub fn download_zip(
    url: &str,
    local_zip_name: &str,
    target_dir: &PathBuf,
    env: &Env,
) -> Result<(), Error> {
    let temp_dir = env.cache_dir.as_path();
    let archive_path = temp_dir.join(local_zip_name);
    let mut reader = ureq::get(url).call()?.into_reader().take(250_000_000);
    let ret = from_reader(&mut reader, &archive_path, target_dir);
    match std::fs::remove_file(&archive_path) {
        Ok(ok) => ok,
        Err(e) => warn!(
            "Cannot remove file {}: {}",
            archive_path.to_string_lossy(),
            e
        ),
    };
    ret
}

pub fn extract_zip(
    source_file: &PathBuf,
    target_dir: &PathBuf,
) -> Result<(), zip::result::ZipError> {
    zip_extract(source_file, target_dir)
}

pub fn copy_file(source_file: &PathBuf, target_file: &PathBuf) -> Result<(), std::io::Error> {
    std::fs::copy(source_file, target_file).and(Ok(()))
}

pub fn rename_file(source_file: &PathBuf, target_file: &PathBuf) -> Result<(), std::io::Error> {
    std::fs::rename(source_file, target_file).and(Ok(()))
}

fn from_reader<R: ?Sized>(
    reader: &mut R,
    archive_path: &PathBuf,
    target_dir: &PathBuf,
) -> Result<(), Error>
where
    R: Read,
{
    let mut file = File::create(&archive_path)?;
    info!("Create file: {}", &archive_path.to_string_lossy());
    std::io::copy(reader, &mut file)?;
    Ok(zip_extract(&archive_path, target_dir)?)
}
