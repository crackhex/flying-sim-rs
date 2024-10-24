#![feature(integer_sign_cast)]

mod bruteforce;
mod includes;
mod simulations;
mod tests;
mod utils;

use crate::includes::mario_state::{pack_input_u8, simulate_inputs};
use crate::simulations::object_collision::CylinderHitbox;
use crate::utils::file_handling::{DumpFile, InputFile};
use includes::mario_state::MarioState;
use std::path::Path;
use std::sync::Arc;

fn main() {
    let path: &Path = Path::new("Path\\To\\File\\dump.json");
    let mut dumpfile = DumpFile::read_file(path);
    let mut mario_state = MarioState::default();
    let mut x = dumpfile.unwrap();
    //println!("{:?}", x);
    let coin1 = CylinderHitbox {
        pos: [10.0, 10.0, 10.0],
        radius: 0.0,
        target: 0,
        active: true,
        height: 160,
        index: 0,
    };
    let mut objects: Vec<CylinderHitbox> = vec![coin1];
    let mut input_file: InputFile = x.parse_inputs().unwrap();
    input_file.simulate();
    //println!("{:?}", &input_file.objects);
    //mario_state.collect_closest_object(&mut input_file.objects);
    //for obj in input_file.objects.iter() {
    // println! {"{:?}", obj.active}
    //}
    let inputs: Arc<[i16]> = Arc::new([
        pack_input_u8([0, 13]),
        pack_input_u8([0, 13]),
        pack_input_u8([0, 13]),
        pack_input_u8([0, 13]),
    ]);
    simulate_inputs(&mut mario_state, inputs);
}
