#![feature(integer_sign_cast)]

mod includes;
mod tests;
use includes::mario_state::MarioState;

fn main() {
    let mut mario_state = MarioState::new();
    mario_state.update_state(127, 126);
    mario_state.update_state(127,126);

}
