#![feature(integer_sign_cast)]

mod includes;
use includes::mario_state::MarioState;

fn main() {
    let mut mario_state = MarioState::new();
    mario_state.update_state(0, 0);
    mario_state.update_state(0,0);

}
