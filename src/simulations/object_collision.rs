use serde::{Deserialize, Serialize};

pub trait Interact {
    fn is_active(&self) -> bool;
    fn horizontal_dist_to_mario(&self, _: &[f32]) -> f32;
    fn vertical_dist_to_mario(&self, _: &[f32]) -> f32;
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CylinderHitbox {
    pub pos: [f32; 3],
    pub radius: f32,
    pub target: u16,
    pub active: bool,
    pub height: u32,
}
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CuboidHitbox {
    pub vertices: [[i32; 2]; 4],
    pub height: i32,
    pub minimum: i32,
    pub active: bool,
}
pub enum Hitboxes {
    Cylinder(CylinderHitbox),
    Cuboid(CuboidHitbox),
}
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Targets<T>
where
    T: Interact,
{
    pub(crate) data: Vec<T>,
}
impl Interact for CylinderHitbox {
    fn is_active(&self) -> bool {
        self.active
    }

    fn horizontal_dist_to_mario(&self, pos: &[f32]) -> f32 {
        let dist: f32 = ((self.pos[0] - pos[0]) * (self.pos[0] - pos[0])
            + (self.pos[2] - pos[2]) * (self.pos[2] - pos[2]))
            .sqrt();
        dist
    }
    fn vertical_dist_to_mario(&self, pos: &[f32]) -> f32 {
        todo!()
    }
}

impl Interact for CuboidHitbox {
    fn is_active(&self) -> bool {
        self.active
    }
    fn horizontal_dist_to_mario(&self, pos: &[f32]) -> f32 {
        todo!()
    }
    fn vertical_dist_to_mario(&self, pos: &[f32]) -> f32 {
        todo!()
    }
}
