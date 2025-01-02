use crate::includes::mario_state::MarioState;
use crate::includes::trig_table::{approach_f32, coss, sins};

pub const fn update_air_without_turn(m: &mut MarioState) {
    let mut sideways_speed: f32 = 0.0;
    let mut intended_d_yaw: i16 = 0;
    let mut intended_mag: f32 = 0.0;
    let drag_threshold: f32 = 32.0; // not simulating long jump so 32.0f
    m.forward_vel = approach_f32(m.forward_vel, 0.0f32, 0.35f32, 0.35f32);
    if m.controller.stick_x != 0.0 && m.controller.stick_y != 0.0 {
        intended_d_yaw = m.intended_yaw - m.face_angle[1];
        intended_mag = m.intended_mag / 32.0;
        m.forward_vel += intended_mag * coss(intended_d_yaw) * 1.5;
        sideways_speed = intended_mag * sins(intended_d_yaw) * 1.5;
    }

    // Uncapped air speed. Net positive when moving forward.
    if m.forward_vel > drag_threshold {
        m.forward_vel -= 1.0;
    }
    if m.forward_vel < 16.0 {
        m.forward_vel += 2.0;
    }
    m.slide_vel_x = m.forward_vel * sins(m.face_angle[1]);
    m.slide_vel_z = m.forward_vel * coss(m.face_angle[1]);

    m.slide_vel_x += sideways_speed * sins(m.face_angle[1].wrapping_add(0x4000));
    m.slide_vel_z += sideways_speed * coss(m.face_angle[1].wrapping_add(0x4000));

    m.vel[0] = m.slide_vel_x;
    m.vel[2] = m.slide_vel_z;
}
