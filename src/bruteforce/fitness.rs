use crate::includes::mario_state::MarioState;
use crate::simulations::object_collision::{Interact, Targets};

pub fn calculate_fitness<T: Interact>(m: &MarioState, targets: &Targets<T>) {}
