#![feature(integer_sign_cast)]

mod includes;
mod tests;
mod simulations;
mod utils;

use std::path::Path;
use std::sync::Arc;
use serde::Deserialize;
use includes::mario_state::MarioState;
use crate::includes::mario_state::{pack_input_u8, simulate_inputs};
use crate::simulations::object_collision::Object;
use crate::utils::file_handling::{DumpFile, InputFile};

fn main() {
    let path: &Path = Path::new("Path\\TO\\dump.json");
    let mut dumpfile = DumpFile::read_file(path);
    let mut mario_state = MarioState::default();
    let x = dumpfile.unwrap();
    println!("{:?}", x);
    let coin1 = Object {
        pos: [10.0, 10.0, 10.0],
        radius: 0.0,
        target: 0,
        active: true,
    };
    let mut objects: Vec<Object> = vec![coin1];
    let mut input_file = InputFile::default();
    input_file.objects = objects;
    println!("{:?}", input_file.objects);
    mario_state.collect_closest_object(&mut input_file.objects);
    for obj in input_file.objects.iter() {
        println! {"{:?}", obj.active}
    }
    let inputs: Arc<[i16]> = Arc::new([pack_input_u8([0, 13]), pack_input_u8([0, 13]), pack_input_u8([0, 13]),pack_input_u8([0, 13])]);
    simulate_inputs(&mut mario_state, inputs);
}
