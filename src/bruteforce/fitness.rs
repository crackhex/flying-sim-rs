use crate::includes::mario_state::MarioState;
use crate::simulations::target_interaction::{CylinderHitbox, Interact, Targets};

#[derive(Default, Debug, Clone)]
pub struct Segment {
    // Dymamic dispatch for targets? Probably not a good idea for performance.
    pub inputs: Vec<i16>,
    pub initial_state: MarioState,
    pub targets: Targets,
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
    mario_state: &mut MarioState,
    targets: &mut Targets,
    goal: &impl Interact,
    inputs: &[i16],
) -> f32 {
    println!("{:?}", mario_state);
    inputs.iter().for_each(|input| {
        mario_state.update_flying(input);
        mario_state.hit_closest_target(targets);
    });
    println!("{:?}", mario_state);
    calculate_fitness(mario_state, targets, goal, inputs.len())
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

pub fn generate_segments(
    mario_state: &mut MarioState,
    targets: &Targets,
    inputs: &[i16],
) -> Vec<Segment> {
    let mut segment_list: Vec<Segment> = vec![];
    let segment_size: usize = 40;
    let num_segments: usize = inputs.len().div_ceil(segment_size);
    let mut current_segment: usize = 0;
    (0..num_segments).for_each(|_| {
        let segment = Segment {
            targets: targets.clone(),
            ..Default::default()
        };
        segment_list.push(segment);
    });

    // Will make short extra segments not happen, maybe append to the previous segment
    for (i, input) in inputs.iter().enumerate() {
        if i % segment_size == 0 && i != 0 {
            let segment_targets = segment_list[current_segment].targets.list_inactive();
            segment_list[current_segment].initial_state = *mario_state;
            segment_list[current_segment].targets = segment_targets;
            current_segment += 1
        }
        mario_state.update_flying(input);
        mario_state.hit_closest_target(&mut segment_list[current_segment].targets);
        segment_list[current_segment].inputs.push(*input);
    }
    let segment_targets = segment_list[current_segment].targets.list_inactive();
    segment_list[current_segment].targets = segment_targets;
    segment_list
}
