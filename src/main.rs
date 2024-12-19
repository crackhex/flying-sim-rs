#![feature(integer_sign_cast)]
#![feature(int_roundings)]
#![feature(ascii_char)]
extern crate core;

mod bruteforce;
mod includes;
mod simulations;
mod tests;
mod utils;

use crate::bruteforce::bruteforce_loop::mario_bruteforce;
use crate::simulations::target_interaction::Targets;
use crate::utils::file_handling::{DumpFile, InputFile};
use std::path::Path;

fn main() {
    let dump_path: &Path = Path::new("dump.json");
    let target_path: &Path = Path::new("targets.json");
    let mut dump_file = DumpFile::read_file(dump_path).unwrap();
    let targets = Targets::read_file(target_path).unwrap();
    let mut input_file: InputFile = dump_file.parse_inputs().unwrap();
    input_file.targets = targets;
    println!("{:?}", input_file.inputs.len());
    input_file.inputs.pop();
    let x = mario_bruteforce(input_file).unwrap();
    x.write_file(Path::new("inputs.json")).unwrap();
    //simulate(&mut input_file);
}
