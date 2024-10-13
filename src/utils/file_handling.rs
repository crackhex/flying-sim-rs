use std::fs::File;
use std::io::Write;
use std::path::{Path};
use crate::includes::mario_state::MarioState;
use crate::simulations::surface_collision::Surface;
use crate::simulations::object_collision::Object;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Deserializer;
use thiserror::Error;

#[derive(Serialize, Deserialize, Default)]
pub struct InputFile {
    pub initial_conditions: MarioState,
    pub objects: Vec<Object>,
    pub inputs: Vec<i16>,
    pub tri_list: Vec<Surface>,
}


#[derive(Debug, Error)]
pub enum InputFileError {
    #[error("File cannot be read")]
    FileError(#[from] std::io::Error),
    #[error("Json cannot be deserialised")]
    JsonError(#[from] serde_json::Error),
}

impl InputFile {
    pub fn read_file(&mut self, path_buf: &Path) -> Result<InputFile, InputFileError> {
        let file = File::open(path_buf)?;
        let mut de = Deserializer::from_reader(&file);
        Ok(InputFile::deserialize(&mut de)?)
    }
    pub fn write_file(&self, path: &Path) -> Result<usize, InputFileError> {
        let mut file = File::create(path)?;
        let x = serde_json::to_string(&self)?;
        Ok(file.write(x.as_bytes())?)
    }
}