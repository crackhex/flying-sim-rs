use crate::includes::mario_state::{pack_input, MarioState};
use crate::simulations::object_collision::Object;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct InputFile {
    pub initial_state: MarioState,
    pub objects: Vec<Object>,
    pub inputs: Vec<i16>,
    pub states: Vec<MarioState>,
}

impl InputFile {
    pub fn initial_mario_state(&self) {}
    pub fn simulate(mut self) {
        let m = &mut self.initial_state;
        let x = self.inputs;
        for (i, input) in x.iter().enumerate() {
            m.update_flying(input);
            if i < 20 {
                println!(
                    "{:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                    m.pos,
                    m.face_angle[1],
                    self.states[i + 1].pos,
                    self.states[i + 1].face_angle[1],
                    ((input >> 8i16) & 0xFF) as i8,
                    (input & 0xFF) as i8,
                    m.controller.stick_x
                );
            }
        }
        //simulate_inputs(mario, x.into());
    }
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
    X: i8,
    Y: i8,
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
    data: Vec<DumpStruct>,
}

#[derive(Debug, Error)]
pub enum InputFileError {
    #[error("File cannot be read")]
    FileError(#[from] std::io::Error),
    #[error("Json cannot be deserialised")]
    JsonError(#[from] serde_json::Error),
    #[error("Error retrieving data")]
    DataError,
}

impl DumpFile {
    pub fn read_file(path_buf: &Path) -> Result<DumpFile, InputFileError> {
        let file = File::open(path_buf)?;
        let mut de = Deserializer::from_reader(&file);

        Ok(DumpFile::deserialize(&mut de)?)
    }
    pub fn write_file(&self, path: &Path) -> Result<usize, InputFileError> {
        let mut file = File::create(path)?;
        let x = serde_json::to_string(&self)?;
        Ok(file.write(x.as_bytes())?)
    }
    pub fn parse_inputs(&mut self) -> Result<InputFile, InputFileError> {
        let data = &self.data;
        let first = data.first().ok_or(InputFileError::DataError)?;
        let mut initial_state = MarioState::default();
        initial_state.pos = [
            first.memory.mario_x,
            first.memory.mario_y,
            first.memory.mario_z,
        ];
        initial_state.face_angle[1] = first.memory.mario_facing_yaw as i16;
        initial_state.face_angle[0] = -1024;
        let mut input_file = InputFile {
            initial_state,
            ..Default::default()
        };

        for info in data.iter() {
            let mut state = MarioState::default();
            state.pos = [
                info.memory.mario_x,
                info.memory.mario_y,
                info.memory.mario_z,
            ];
            state.face_angle[1] = info.memory.mario_facing_yaw.cast_signed();
            let input = pack_input(info.input.X, info.input.Y);

            input_file.inputs.push(input);
            input_file.states.push(state);
        }
        Ok(input_file)
    }
}
