use crate::includes::mario_state::{MarioState, pack_input};
use crate::simulations::target_interaction::Targets;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct InputFile {
    pub initial_state: MarioState,
    pub targets: Targets,
    pub inputs: Vec<i16>,
}
impl InputFile {
    pub fn read_file(&self, path: &Path) -> Result<InputFile, InputFileError> {
        let file = File::open(path)?;
        let mut de = Deserializer::from_reader(&file);

        Ok(InputFile::deserialize(&mut de)?)
    }
    pub fn write_file(&self, path: &Path) -> Result<usize, InputFileError> {
        let mut file = File::create(path)?;
        let x = serde_json::to_string(&self)?;
        Ok(file.write(x.as_bytes())?)
    }
    pub fn write_to_m64(&self, path: &Path) -> Result<(), InputFileError> {
        let mut file = File::create(path)?;
        // TODO: Write to m64 file
        Ok(())
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
    mario_pitch: i16,
    mario_yaw_vel: i16,
    mario_pitch_vel: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DumpInputs {
    #[serde(rename = "X")]
    x: i8,
    #[serde(rename = "Y")]
    y: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DumpStruct {
    sample: u32,
    input: DumpInputs,
    memory: DumpMemory,
    frame: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DumpFile {
    data: Vec<DumpStruct>,
}

#[derive(Debug, Error)]
pub enum InputFileError {
    #[error("File cannot be read")]
    File(#[from] std::io::Error),
    #[error("Json cannot be deserialised")]
    Json(#[from] serde_json::Error),
    #[error("Error retrieving data")]
    Data,
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
        let first = data.first().ok_or(InputFileError::Data)?;
        let mut initial_state = MarioState::default();
        initial_state.pos = [
            first.memory.mario_x,
            first.memory.mario_y,
            first.memory.mario_z,
        ];
        initial_state.face_angle[1] = first.memory.mario_facing_yaw.cast_signed();
        initial_state.face_angle[0] = first.memory.mario_pitch;
        initial_state.forward_vel = first.memory.mario_f_speed;
        initial_state.vel[1] = first.memory.mario_v_speed;
        initial_state.angle_vel = [first.memory.mario_pitch_vel, first.memory.mario_yaw_vel, 0];
        let mut input_file = InputFile {
            initial_state,
            ..Default::default()
        };
        data.iter().for_each(|info| {
            let mut state = MarioState::default();
            state.pos = [
                info.memory.mario_x,
                info.memory.mario_y,
                info.memory.mario_z,
            ];
            state.face_angle[1] = info.memory.mario_facing_yaw.cast_signed();
            state.face_angle[0] = info.memory.mario_pitch;
            let input = pack_input(info.input.x, info.input.y);
            input_file.inputs.push(input);
        });
        Ok(input_file)
    }
}
