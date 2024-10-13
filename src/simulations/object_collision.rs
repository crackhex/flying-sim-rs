use serde::{Deserialize, Serialize};
use crate::includes::mario_state::MarioState;

pub trait Interact {
    fn dist_to_mario(&self, _: &MarioState) -> f32;
}

#[derive(Default, Serialize, Deserialize)]
pub struct Object {
    pub pos: [f32; 3],
    pub radius: f32,
    pub target: u16,
    pub active: bool,
}

impl Interact for Object {
    fn dist_to_mario(&self, m: &MarioState) -> f32 {
        let dist: f32 = ((self.pos[0] - m.pos[0])*(self.pos[0] - m.pos[0])
            + (self.pos[2] - m.pos[2])*(self.pos[2] - m.pos[2])).sqrt();
        dist
    }
}






