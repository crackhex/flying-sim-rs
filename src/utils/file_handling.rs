use crate::includes::mario_state::MarioState;
use crate::simulations::object_collision::Object;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Deserializer as De;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct InputFile {
    pub initial_conditions: MarioState,
    pub objects: Vec<Object>,
    pub inputs: Vec<i16>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DumpMemory {
    mario_x: f32,
    mario_y: f32,
    mario_z: f32,
    mario_h_speed: f32,
    mario_f_speed: f32,
    mario_v_speed: f32,
    mario_x_sliding_speed: f32,
    mario_z_sliding_speed: f32,
    mario_facing_yaw: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DumpInputs {
    X: i16,
    Y: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DumpStruct {
    varwatch: Vec<String>,
    memory: DumpMemory,
    input: DumpInputs,
    frame: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DumpFile {
    data: Vec<DumpStruct>
}

#[derive(Debug, Error)]
pub enum InputFileError {
    #[error("File cannot be read")]
    FileError(#[from] std::io::Error),
    #[error("Json cannot be deserialised")]
    JsonError(#[from] serde_json::Error),
}

impl DumpFile {
    pub fn read_file(path_buf: &Path) -> Result<DumpFile, InputFileError> {
        let file = File::open(path_buf)?;
        let mut de = De::from_reader(&file);

        Ok(DumpFile::deserialize(&mut de)?)
    }
    pub fn write_file(&self, path: &Path) -> Result<usize, InputFileError> {
        let mut file = File::create(path)?;
        let x = serde_json::to_string(&self)?;
        Ok(file.write(x.as_bytes())?)
    }
}
