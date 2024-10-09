#![feature(integer_sign_cast)]

mod includes;
mod tests;
mod simulations;

use includes::mario_state::MarioState;

fn main() {
    let mut mario_state = MarioState::default();
    mario_state.update_state(0, 0);
    mario_state.update_state(0,0);
    println!("{:?}", mario_state.forward_vel)

}
