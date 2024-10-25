use crate::bruteforce::fitness::calculate_fitness;
use crate::includes::mario_state::{MarioState, pack_input, simulate_inputs, unpack_input_i8};
use crate::simulations::object_collision::{CylinderHitbox, Targets};
use rand::Rng;

pub fn perturb_inputs(
    mario_state: &mut MarioState,
    targets: &mut Targets,
    goal: CylinderHitbox,
    mut inputs: Vec<i16>,
    fitness: &mut f32,
) -> f32 {
    // Randomly perturb
    let perturbation_freq = 0.15;
    let perturbation_perm = 1;
    let len = inputs.len();
    for i in 0..rand::thread_rng().gen_range(0..(((len - 1) as f32 * perturbation_freq) as usize)) {
        inputs[i] = {
            let mut inputs = unpack_input_i8(inputs[i]);
            let _ = inputs
                .iter()
                .enumerate()
                .map(move |(i, input)| inputs[i] = (*input).wrapping_add(perturbation_perm));
            pack_input(inputs[0], inputs[1])
        }
    }

    simulate_inputs(mario_state, targets, inputs.as_slice());

    let new_fitness = calculate_fitness(mario_state, goal, len - 1);
    if *fitness < new_fitness {
        *fitness
    } else {
        new_fitness
    }
}
