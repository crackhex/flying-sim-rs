#![feature(integer_sign_cast)]

mod includes;
mod tests;
mod simulations;
mod utils;

use includes::mario_state::MarioState;
fn main() {
    let mut mario_state = MarioState::default();
    mario_state.update_flying([0,13]);
    mario_state.update_flying([0,13]);
    mario_state.update_flying([0,13]);
    mario_state.update_flying([0,13]);
}
