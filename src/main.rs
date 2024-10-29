#![feature(integer_sign_cast)]

mod bruteforce;
mod includes;
mod simulations;
mod tests;
mod utils;

use crate::bruteforce::perturbation::mario_bruteforce;
use crate::includes::mario_state::simulate;
use crate::simulations::object_collision::Targets;
use crate::utils::file_handling::{DumpFile, InputFile};
use std::path::Path;

fn main() {
    let dump_path: &Path = Path::new("C:\\Users\\austi\\Desktop\\rust\\flying-sim-rs\\dump.json");
    let target_path: &Path =
        Path::new("C:\\Users\\austi\\Desktop\\rust\\flying-sim-rs\\targets.json");
    let mut dump_file = DumpFile::read_file(dump_path).unwrap();
    let mut targets = Targets::read_file(target_path).unwrap();
    let mut input_file: InputFile = dump_file.parse_inputs().unwrap();
    input_file.targets = targets;
    let m = &input_file.initial_state;
    let goal = input_file.targets.cylinder[input_file.targets.cylinder.len() - 1];
    println!("{:?}", input_file.inputs.len());
    mario_bruteforce(m, input_file.targets, goal, input_file.inputs);
    //simulate(&mut input_file);
}
