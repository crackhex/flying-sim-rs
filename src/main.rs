#![feature(integer_sign_cast)]

mod bruteforce;
mod includes;
mod simulations;
mod tests;
mod utils;

use crate::includes::mario_state::simulate;
use crate::simulations::object_collision::{CylinderHitbox, Targets};
use crate::utils::file_handling::{DumpFile, InputFile};
use std::path::Path;

fn main() {
    let dump_path: &Path = Path::new("Path\\To\\dump.json");
    let target_path: &Path = Path::new("Path\\To\\targets.json");
    let mut dumpfile = DumpFile::read_file(dump_path);

    let mut x = dumpfile.unwrap();
    let coin1 = CylinderHitbox {
        pos: [-3500.0, 0.0, 100.0],
        radius: 150.0,
        active: true,
        height: 160,
        index: 0,
    };
    let coin2 = CylinderHitbox {
        pos: [-3500.0, 0.0, -100.0],
        radius: 150.0,
        active: true,
        height: 0,
        index: 0,
    };
    let mut objects: Vec<CylinderHitbox> = vec![coin1, coin2];
    let mut input_file: InputFile = x.parse_inputs().unwrap();
    let mut targets = Targets {
        cylinder: objects,
        cuboid: vec![],
    };
    input_file.targets = targets;
    simulate(&mut input_file);
    input_file.targets.save_file(target_path).expect("TODO: panic message");
}
