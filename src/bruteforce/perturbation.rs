use crate::bruteforce::fitness::calculate_fitness;
use crate::includes::mario_state::{MarioState, pack_input, unpack_input_i8};
use crate::simulations::object_collision::{CylinderHitbox, Targets};
use rand::Rng;

pub fn perturb_inputs(
    mario_state: &mut MarioState,
    targets: &mut Targets,
    goal: &CylinderHitbox,
    inputs: &mut Vec<i16>,
    fitness: &mut f32,
) {
    // Randomly perturb
    let mut new_inputs = inputs.clone();
    let perturbation_freq = 0.15;
    let perturbation_perm = 1;
    let kj = ((new_inputs.len() - 1) as f32 * perturbation_freq) as usize;
    for i in 0..new_inputs.len() {
        if i % kj == 0 {
            let k = rand::thread_rng().gen_range(0..kj);
            new_inputs[k] = {
                let mut input = unpack_input_i8(new_inputs[k]);
                input[0] = input[0].wrapping_add(perturbation_perm);
                input[1] = input[1].wrapping_add(perturbation_perm);
                pack_input(input[0], input[1])
            };
        }
    }
    let mut break_frame: usize = 1;
    for (i, input) in new_inputs.iter().enumerate() {
        mario_state.update_flying(input);
        mario_state.hit_closest_target(targets);
        if mario_state.hit_goal(goal) {
            if targets.all_targets_inactive() {
                break_frame = i;
                break;
            }
        }
    }

    //simulate_inputs(mario_state, targets, &new_inputs);
    
    let new_fitness = calculate_fitness(mario_state, targets, goal, break_frame);
    if new_fitness < *fitness {
        println! {"{:?}, len: {:?}", new_fitness, break_frame};
        *fitness = new_fitness;
        *inputs = new_inputs;       
    }
}
pub fn mario_bruteforce(
    initial_state: &MarioState,
    targets: Targets,
    goal: CylinderHitbox,
    mut inputs: Vec<i16>,
) {
    let mut f = f32::MAX;
    println!("{:?}", goal);
    let mut mario_first = *initial_state;
    let mut targets_first = targets.clone();
    f = initial_check(&mut mario_first, &mut targets_first, &goal, &inputs);
    println!("{:?}", f);
    loop {
        let mut m = *initial_state;
        let mut target = targets.clone();
        perturb_inputs(&mut m, &mut target, &goal, &mut inputs, &mut f);
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
