use crate::utils::file_handling::InputFileError;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::fs::File;
use std::path::Path;

pub trait Interact {
    fn is_active(&self) -> bool;
    fn is_mario_in_bounds(&self, _: [f32; 3]) -> bool;
    fn horizontal_dist_to_mario(&self, _: [f32; 3]) -> f32;
    fn vertical_dist_to_mario(&self, _: [f32; 3]) -> f32;
}

#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone)]
pub struct CylinderHitbox {
    pub pos: [f32; 3],
    pub radius: f32,
    pub active: bool,
    pub height: u32,
    pub index: u32,
}
#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone)]
pub struct CuboidHitbox {
    pub vertices: [[i32; 2]; 4],
    pub height: i32,
    pub minimum: i32,
    pub active: bool,
    pub index: u32,
}
pub enum Hitboxes {
    Cylinder(CylinderHitbox),
    Cuboid(CuboidHitbox),
}
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Targets {
    pub cuboid: Vec<CuboidHitbox>,
    pub cylinder: Vec<CylinderHitbox>,
}
impl Targets {
    pub fn read_file(path: &Path) -> Result<Targets, InputFileError> {
        let file = File::open(path)?;
        let mut de = Deserializer::from_reader(file);
        Ok(Targets::deserialize(&mut de)?)
    }
    pub fn save_file(&self, path: &Path) -> Result<(), InputFileError> {
        let file = File::create(path)?;
        Ok(serde_json::to_writer(file, &self)?)
    }
    pub fn all_targets_inactive(&self) -> bool {
        for hitbox in self.cuboid.iter() {
            if hitbox.is_active() {
                return false;
            }
        }
        for hitbox in self.cylinder.iter() {
            if hitbox.is_active() {
                return false;
            }
        }
        true
    }
}

impl Interact for CylinderHitbox {
    fn is_active(&self) -> bool {
        self.active
    }
    fn is_mario_in_bounds(&self, mario_pos: [f32; 3]) -> bool {
        if mario_pos[1] >= self.pos[1] - 160.0f32
            && mario_pos[1] <= self.pos[1] + 64.0f32
            && self.horizontal_dist_to_mario(mario_pos) <= self.radius
        {
            return true;
        }
        false
    }
    fn horizontal_dist_to_mario(&self, mario_pos: [f32; 3]) -> f32 {
        let dist: f32 = ((self.pos[0] - mario_pos[0]) * (self.pos[0] - mario_pos[0])
            + (self.pos[2] - mario_pos[2]) * (self.pos[2] - mario_pos[2]))
            .sqrt();
        dist
    }
    fn vertical_dist_to_mario(&self, mario_pos: [f32; 3]) -> f32 {
        todo!()
    }
}

impl Interact for &CylinderHitbox {
    fn is_active(&self) -> bool {
        self.active
    }
    fn is_mario_in_bounds(&self, mario_pos: [f32; 3]) -> bool {
        if mario_pos[1] >= self.pos[1] - 160.0f32
            && mario_pos[1] <= self.pos[1] + 64.0f32
            && self.horizontal_dist_to_mario(mario_pos) <= self.radius
        {
            return true;
        }
        false
    }
    fn horizontal_dist_to_mario(&self, mario_pos: [f32; 3]) -> f32 {
        let dist: f32 = ((self.pos[0] - mario_pos[0]) * (self.pos[0] - mario_pos[0])
            + (self.pos[2] - mario_pos[2]) * (self.pos[2] - mario_pos[2]))
            .sqrt();
        dist
    }
    fn vertical_dist_to_mario(&self, mario_pos: [f32; 3]) -> f32 {
        (self.pos[1] - mario_pos[1]).abs()
    }
}

impl Interact for CuboidHitbox {
    fn is_active(&self) -> bool {
        self.active
    }
    fn is_mario_in_bounds(&self, _: [f32; 3]) -> bool {
        todo!()
    }
    fn horizontal_dist_to_mario(&self, mario_pos: [f32; 3]) -> f32 {
        todo!()
    }
    fn vertical_dist_to_mario(&self, mario_pos: [f32; 3]) -> f32 {
        todo!()
    }
}
