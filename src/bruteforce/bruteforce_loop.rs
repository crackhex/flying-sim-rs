use crate::bruteforce::fitness::{calculate_fitness, generate_segments, initial_fitness};
use crate::bruteforce::perturbation::perturb_inputs;
use crate::includes::mario_state::MarioState;
use crate::utils::file_handling::{InputFile, InputFileError};
use crate::utils::m64_handling::{M64File, construct_inputs_i16};

pub fn mario_bruteforce(mut input_file: InputFile) -> Result<InputFile, InputFileError> {
    let mut initial_state: MarioState = input_file.initial_state;
    let segments = generate_segments(&mut initial_state, &input_file.targets, &input_file.inputs);
    let goal = &input_file.targets.cylinder[input_file.targets.cylinder.len() - 1];
    let mut mario_first = input_file.initial_state;
    let mut targets_first = input_file.targets.clone();
    let mut fitness = initial_fitness(
        &mut mario_first,
        &mut targets_first,
        goal,
        &input_file.inputs,
    );
    let initial_len = input_file.inputs.len();
    let initial_frame = 0; // TODO: Implement
    println!("{:?}", fitness);
    let mut m64_file = M64File::read_file("inputs.m64".as_ref()).unwrap_or_else(|_| M64File::new());
    loop {
        let mut new_inputs = input_file.inputs.clone();
        perturb_inputs(&mut new_inputs);
        let mut targets = input_file.targets.clone();
        let mut mario_state = input_file.initial_state;
        let mut break_frame: usize = 1;
        for (i, input) in new_inputs.iter().enumerate() {
            mario_state.update_flying(input);
            mario_state.hit_closest_target(&mut targets);
            if mario_state.hit_goal(goal) && targets.all_inactive() {
                break_frame = i + 1;
                break;
            }
        }
        let new_fitness = calculate_fitness(&mario_state, &targets, goal, break_frame);
        if new_fitness < fitness && break_frame > 1 {
            println! {"{:?}, len: {:?}", new_fitness, break_frame};
            fitness = new_fitness;
            let m64_inputs = construct_inputs_i16(&new_inputs);
            input_file.inputs = new_inputs;
            //m64_file.replace_inputs(
            // &(initial_frame..(initial_frame + initial_len)),
            // &(initial_frame..(initial_frame + break_frame)),
            // &m64_inputs
            // ).expect("TODO: panic message"); TODO Implement
            let _ = input_file.write_file("inputs.json".as_ref());
        }
    }
}
