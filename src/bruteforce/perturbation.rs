use crate::bruteforce::fitness::calculate_fitness;
use crate::includes::mario_state::{MarioState, pack_input, unpack_input_i8};
use crate::simulations::object_collision::{CylinderHitbox, Interact, Targets};
use rand::Rng;

pub fn perturb_inputs<T>(
    mario_state: &mut MarioState,
    targets: &mut Targets,
    goal: &T,
    inputs: &mut [i16],
    fitness: &mut f32,
) -> bool
where
    for<'a> &'a T: Interact,
{
    // Randomly perturb
    let perturbation_freq = 0.16; // 0 to 1
    let perturbation_perm = 3; // 1 to
    let freq = ((inputs.len() as f32 * (1f32 - perturbation_freq)) / 3.0f32) as usize;
    for i in 0..inputs.len() {
        if i % freq == 0 {
            let k = rand::thread_rng().gen_range(0..inputs.len());
            inputs[k] = {
                let mut input = unpack_input_i8(inputs[k]);
                input[0] = input[0].wrapping_add(
                    rand::thread_rng().gen_range(-perturbation_perm..perturbation_perm),
                );
                input[1] = input[1].wrapping_add(
                    rand::thread_rng().gen_range(-perturbation_perm..perturbation_perm),
                );
                pack_input(input[0], input[1])
            };
        }
    }
    let mut break_frame: usize = 1;
    for (i, input) in inputs.iter().enumerate() {
        mario_state.update_flying(input);
        mario_state.hit_closest_target(targets);
        if mario_state.hit_goal(goal) && targets.all_targets_inactive() {
            break_frame = i + 1;
            break;
        }
    }

    //simulate_inputs(mario_state, targets, &new_inputs);

    let new_fitness = calculate_fitness(mario_state, targets, goal, break_frame);
    if new_fitness < *fitness && break_frame > 1 {
        println! {"{:?}, len: {:?}", new_fitness, break_frame};
        *fitness = new_fitness;
        return true;
    }
    false
}
pub fn mario_bruteforce(
    initial_state: &MarioState,
    targets: Targets,
    goal: CylinderHitbox,
    mut inputs: Vec<i16>,
) {
    println!("{:?}", goal);
    let mut mario_first = *initial_state;
    let mut targets_first = targets.clone();
    let mut fitness = initial_check(&mut mario_first, &mut targets_first, &goal, &inputs);
    println!("{:?}", fitness);
    loop {
        let mut m = *initial_state;
        let mut new_inputs = inputs.clone();
        let mut target = targets.clone();
        if perturb_inputs(&mut m, &mut target, &goal, &mut new_inputs, &mut fitness) {
            inputs = new_inputs
        };
    }
}

fn initial_check(
    m: &mut MarioState,
    targets: &mut Targets,
    goal: &CylinderHitbox,
    inputs: &[i16],
) -> f32 {
    inputs.iter().for_each(|input| {
        m.update_flying(input);
        m.hit_closest_target(targets);
    });
    calculate_fitness(m, targets, goal, inputs.len())
}
