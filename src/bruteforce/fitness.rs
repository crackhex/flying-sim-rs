use crate::includes::mario_state::MarioState;
use crate::simulations::target_interaction::{CylinderHitbox, Interact, Targets};

pub struct Segment {
    // Dymamic dispatch for targets? Probably not a good idea for performance.
    pub inputs: Vec<i16>,
    pub initial_state: MarioState,
    pub target: CylinderHitbox,
    pub fitness: f32,
}
// Extremely basic fitness function
// The target passed in here is the target mario is aiming for
// The frame_count passed is the number of frames in the run
pub fn calculate_fitness(
    m: &MarioState,
    targets: &Targets,
    goal: &impl Interact,
    frame_count: usize,
) -> f32 {
    for x in targets.cylinder.iter() {
        if x.active {
            return f32::MAX;
        }
    }
    let dist = goal.horizontal_dist_to_mario(m.pos);
    let fitness = (frame_count as f32) * 100.0f32 + dist;

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

pub fn initial_fitness(
    m: &mut MarioState,
    targets: &mut Targets,
    goal: &impl Interact,
    inputs: &[i16],
) -> f32 {
    inputs.iter().for_each(|input| {
        m.update_flying(input);
        m.hit_closest_target(targets);
    });
    calculate_fitness(m, targets, goal, inputs.len())
}
pub fn generate_targets(
    initial_state: &mut MarioState,
    inputs: &[i16],
    targets: &Targets,
    length: u32,
) -> Targets {
    let x = targets.cylinder.iter();
    let cylinders: Vec<CylinderHitbox> = vec![];
    inputs
        .iter()
        .enumerate()
        .for_each(|(frame, input)| if (frame as u32) < length {});
    Targets {
        cuboid: vec![],
        cylinder: vec![],
    }
}

pub fn generate_segments(initial_state: &mut MarioState, inputs: &[i16], target: CylinderHitbox) {}
