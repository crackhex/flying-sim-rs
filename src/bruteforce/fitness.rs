use crate::includes::mario_state::MarioState;
use crate::simulations::object_collision::{CylinderHitbox, Interact, Targets};

// Extremely basic fitness function
// The target passed in here is the target mario is aiming for
// The frame_count passed is the number of frames in the run
pub fn calculate_fitness(m: &MarioState, target: impl Interact, frame_count: usize) -> f32 {
    let dist = target.horizontal_dist_to_mario(m.pos);
    let fitness = (frame_count as f32) + dist;
    fitness
}

pub fn final_target(targets: &Targets) -> Option<&CylinderHitbox> {
    let final_index = (targets.cylinder.len() + targets.cuboid.len()) as u32;
    // Iterate over all the Cylindrical Targets, find the last target
    targets
        .cylinder
        .iter()
        .find(|&i| i.index.cmp(&final_index).is_eq())
}
