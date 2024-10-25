#![feature(integer_sign_cast)]

mod bruteforce;
mod includes;
mod simulations;
mod tests;
mod utils;

use crate::includes::mario_state::simulate;
use crate::simulations::object_collision::Targets;
use crate::utils::file_handling::{DumpFile, InputFile};
use std::path::Path;

fn main() {
    let dump_path: &Path = Path::new("Path\\To\\dump.json");
    let target_path: &Path = Path::new("Path\\To\\targets.json");
    let mut dump_file = DumpFile::read_file(dump_path).unwrap();
    let mut targets = Targets::read_file(target_path).unwrap();
    let mut input_file: InputFile = dump_file.parse_inputs().unwrap();
    input_file.targets = targets;
    simulate(&mut input_file);
}
