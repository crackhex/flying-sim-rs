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

#[derive(Default, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CylinderHitbox {
    pub pos: [f32; 3],
    pub radius: f32,
    pub active: bool,
    pub height: u32,
    pub index: u32,
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

#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone)]
pub struct CuboidHitbox {
    pub pos: [i32; 3],
    pub side_length: [i32; 2],
    pub height: i32,
    pub active: bool,
    pub index: u32,
}
impl Interact for CuboidHitbox {
    fn is_active(&self) -> bool {
        self.active
    }
    fn is_mario_in_bounds(&self, mario_pos: [f32; 3]) -> bool {
        if mario_pos[0] > self.pos[0] as f32
            && mario_pos[0] < (self.pos[0] + self.side_length[0]) as f32
            && mario_pos[2] > self.pos[2] as f32
            && mario_pos[2] < (self.pos[2] + self.side_length[1]) as f32
            && mario_pos[1] > self.pos[1] as f32
            && mario_pos[1] <= (self.pos[1] + self.height) as f32
        {
            return true;
        };
        false
    }

    fn horizontal_dist_to_mario(&self, _: [f32; 3]) -> f32 {
        todo!()
    }

    fn vertical_dist_to_mario(&self, mario_pos: [f32; 3]) -> f32 {
        todo!()
    }
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
    pub fn all_inactive(&self) -> bool {
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
    pub fn list_inactive(&self) -> Targets {
        let mut inactive_targets: Targets = Targets::default();
        self.cuboid.iter().for_each(|cuboid| {
            if !cuboid.active {
                inactive_targets.cuboid.push(*cuboid);
            }
        });
        self.cylinder.iter().for_each(|cylinder| {
            if !cylinder.active {
                inactive_targets.cylinder.push(*cylinder);
            }
        });
        inactive_targets
    }
}
