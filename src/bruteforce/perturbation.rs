use crate::includes::mario_state::{pack_input, unpack_input_i8};
use rand::Rng;

pub fn perturb_inputs(inputs: &mut [i16]) {
    // Randomly perturb
    let perturbation_freq = 0.1; // 0 to 1
    let perturbation_perm = 10; // 1 to
    for i in inputs.iter_mut() {
        let rand_float = rand::thread_rng().gen_range(0..100) as f32;
        if rand_float < perturbation_freq * 100.0f32 {
            *i = {
                let mut input = unpack_input_i8(*i);
                input[0] = input[0].wrapping_add(
                    rand::thread_rng().gen_range(-perturbation_perm..perturbation_perm),
                );
                input[1] = input[1].wrapping_add(
                    rand::thread_rng().gen_range(-perturbation_perm..perturbation_perm),
                );
                pack_input(input[0], input[1])
            }
        }
    }
}
