use crate::includes::mario_state::MarioState;
use crate::simulations::object_collision::Interact;

pub fn calculate_fitness(m: &MarioState, target: impl Interact, inputs: Vec<&i16>) {
    let len = inputs.len();
    let dist = target.horizontal_dist_to_mario(m.pos);

}
