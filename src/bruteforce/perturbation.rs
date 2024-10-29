use crate::bruteforce::fitness::calculate_fitness;
use crate::includes::mario_state::{pack_input, unpack_input_i8, MarioState};
use crate::simulations::object_collision::{CylinderHitbox, Targets};
use rand::Rng;

pub fn perturb_inputs(
    mario_state: &mut MarioState,
    targets: &Targets,
    goal: &CylinderHitbox,
    inputs: &mut Vec<i16>,
    fitness: &mut f32,
) -> f32 {
    // Randomly perturb
    let mut new_inputs = inputs.clone();
    let mut targets = targets.clone();
    let perturbation_freq = 0.15;
    let perturbation_perm = 7;
    let len = new_inputs.len();
    for i in 0..len - 1 {
        if i % 10 == 0 {
            let k =
                rand::thread_rng().gen_range(0..(((len - 1) as f32 * perturbation_freq) as usize));
            new_inputs[k] = {
                let mut input = unpack_input_i8(new_inputs[k]);
                input[0] =
                    input[0].wrapping_add(perturbation_perm * rand::thread_rng().gen_range(1..4));
                input[1] =
                    input[1].wrapping_add(perturbation_perm * rand::thread_rng().gen_range(1..4));
                pack_input(input[0], input[1])
            };
        }
    }
    let mut break_frame: usize = 0;
    for (i, input) in new_inputs.iter().enumerate() {
        mario_state.update_flying(input);
        mario_state.hit_closest_target(&mut targets);

        if mario_state.hit_goal(&goal) == true {
            let mut active_targets: bool = false;
            targets.cylinder.iter().for_each(|x| {
                if x.active == true {
                    active_targets = true;
                }
            });
            if !active_targets {
                break_frame = i;
                break;
            }
        }
    }
    //simulate_inputs(mario_state, targets, &new_inputs);

    let new_fitness = calculate_fitness(mario_state, goal, len - 1);
    if new_fitness < *fitness {
        println! {"{:?}, len: {:?}", new_fitness, break_frame + 1};
        *fitness = new_fitness;
        *inputs = new_inputs;
        new_fitness
    } else {
        *fitness
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
    //initial_loop(&mut mario_first, &mut targets_first, &goal, &inputs, &mut f);
    loop {
        let mut m = *initial_state;
        let mut target = targets.clone();
        perturb_inputs(&mut m, &mut target, &goal, &mut inputs, &mut f);
    }
}

fn initial_loop(
    p0: &mut MarioState,
    p1: &mut Targets,
    p2: &CylinderHitbox,
    p3: &[i16],
    p4: &mut f32,
) {
    todo!()
}
