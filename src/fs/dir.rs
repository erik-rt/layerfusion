use log::*;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

pub struct Dir {
    pub contents: Vec<PathBuf>,
    pub path: PathBuf,
}

impl Dir {
    pub fn read_dir(path: PathBuf) -> eyre::Result<Self> {
        info!("Reading current path {path:?}");

        // Get the subdirectories of the input folder
        let mut contents = fs::read_dir(&path)?
            .map(|result| result.map(|item| item.path()))
            .filter(|c| c.as_ref().unwrap().is_dir())
            .collect::<Result<Vec<PathBuf>, _>>()?;

        // Sort the subdirectories in alphanumeric order
        // This is unnecessary since we're using a BTreeMap but I'm keeping it in case we move to a
        // Vec
        // contents.sort();

        Ok(Dir { contents, path })
    }
}

#[derive(Error, Debug)]
pub enum DirError {
    #[error("failed to get file stem: {0}")]
    FileStemError(String),
}
