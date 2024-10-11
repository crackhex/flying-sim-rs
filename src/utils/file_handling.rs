use std::fs::File;
use std::path::{Path};
use crate::includes::mario_state::MarioState;
use crate::simulations::surface_collision::Surface;
use crate::simulations::object_collision::Object;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

#[derive(Serialize, Deserialize)]
pub struct InputFile {
    pub initial_conditions: MarioState,
    pub objects: Vec<Object>,
    pub inputs: Vec<i16>,
    pub tri_list: Vec<Surface>,
}

impl InputFile {
    pub fn read_file(&mut self, path_buf: &Path) -> InputFile {
        let file = match File::open(path_buf) {
            Ok(x) => x,
            Err(_) => todo!(),
        };
        let mut de = Deserializer::from_reader(&file);
        let x = match InputFile::deserialize(&mut de) {
            Ok(u) => u,
            Err(_) => todo!(),
        };
        x
    }
}