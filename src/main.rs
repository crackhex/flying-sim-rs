#![feature(integer_sign_cast)]

mod includes;
mod tests;
mod simulations;
mod utils;

use std::path::Path;
use std::sync::Arc;
use includes::mario_state::MarioState;
use crate::includes::mario_state::simulate_inputs;
use crate::simulations::object_collision::Object;
use crate::utils::file_handling::InputFile;

fn main() {
    let path: &Path = Path::new("C:\\Users\\austi\\Desktop\\rust\\flying-sim-rs\\settings.json");
    let mut inputfile = InputFile::default();
    let _ = inputfile.read_file(path);
    println!("{:?}", inputfile.objects);
    let mut mario_state = MarioState::default();

    let coin1 = Object {
        pos: [10.0, 10.0, 10.0],
        radius: 0.0,
        target: 0,
        active: true,
    };
    let mut objects: Vec<Object> = vec![coin1];
    inputfile.objects = objects;
    println!("{:?}", inputfile.objects);
    mario_state.collect_closest_object(&mut inputfile.objects);
    for obj in inputfile.objects.iter() {
        println! {"{:?}", obj.active}
    }
    let inputs: Arc<[[i8; 2]]> = Arc::new([[0, 13], [0, 13], [0, 13], [0, 13]]);
    simulate_inputs(&mut mario_state, inputs);
    inputfile.write_file(path).expect("TODO: panic message");
}
