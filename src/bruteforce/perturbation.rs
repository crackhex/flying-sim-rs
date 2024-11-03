use crate::includes::mario_state::{pack_input, unpack_input_i8};
use rand::Rng;

pub fn perturb_inputs(inputs: &mut [i16]) {
    // Randomly perturb
    let perturbation_freq = 0.2; // 0 to 1
    let perturbation_perm = 25; // 1 to
    let freq = ((inputs.len() as f32 * (1f32 - perturbation_freq)) / 5.0f32) as usize;
    for i in 0..inputs.len() {
        let k = rand::thread_rng().gen_range(0..inputs.len());
        if (i + k) % freq == 1 {
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
}
