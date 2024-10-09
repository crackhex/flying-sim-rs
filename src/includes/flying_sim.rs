use std::cmp::Ordering;
use crate::includes::mario_state::MarioState;
use crate::includes::trig_table::{approach_value, coss, sins};

pub fn update_flying_yaw(m: &mut MarioState) {

    let target_yaw_vel: i16 = -((m.controller.stick_x * (m.forward_vel / 4.0)) as i16); // 0x4000
    match target_yaw_vel.cmp(&0i16) {
        Ordering::Greater => {
            if m.angle_vel[1] < 0 {
                m.angle_vel[1] += 0x40;
                if m.angle_vel[1] > 0x10 {
                    m.angle_vel[1] = 0x10;
                }
            } else {
                m.angle_vel[1] = approach_value(m.angle_vel[1] as i32, target_yaw_vel as i32, 0x10, 0x20) as i16;
            }
        }
        Ordering::Less => {
            if m.angle_vel[1] > 0 {
                m.angle_vel[1] -= 0x40;
                if m.angle_vel[1] < -0x10 {
                    m.angle_vel[1] = -0x10;
                }
            } else {
                m.angle_vel[1] = approach_value(m.angle_vel[1] as i32, target_yaw_vel as i32, 0x20, 0x10) as i16;
            }
        }
        Ordering::Equal => {
            m.angle_vel[1] = approach_value(m.angle_vel[1] as i32, 0, 0x40, 0x40) as i16;
        }
    }
    m.face_angle[1] = m.face_angle[1].wrapping_add(m.angle_vel[1]);
    m.face_angle[2] = 20_i16.wrapping_mul(-m.angle_vel[1]);
}

pub fn update_flying_pitch(m: &mut MarioState) {
    let target_pitch_vel: i16 = -((m.controller.stick_y * (m.forward_vel / 5.0)) as i16); // 0x4000
    match target_pitch_vel.cmp(&0i16) {
        Ordering::Greater => {
            if m.angle_vel[0] < 0 {
                m.angle_vel[0] += 0x40;
                if m.angle_vel[0] > 0x20 {
                    m.angle_vel[0] = 0x20;
                }
            } else {
                m.angle_vel[0] = approach_value(m.angle_vel[0] as i32, target_pitch_vel as i32, 0x20, 0x40) as i16; // fix with approach_s32 function
            }
        },
        Ordering::Less => {
            if m.angle_vel[0] > 0 {
                m.angle_vel[0] -= 0x40;
                if m.angle_vel[0] < -0x20 {
                    m.angle_vel[0] = -0x20;
                }
            } else {
                m.angle_vel[0] = approach_value(m.angle_vel[0] as i32, target_pitch_vel as i32, 0x40, 0x20) as i16; // fix with approach_s32 function
            }
        },
        Ordering::Equal => {
            m.angle_vel[1] = approach_value(m.angle_vel[0] as i32, 0, 0x40, 0x40) as i16; // fix with approach_s32 function
        },
    }
}

pub fn update_flying(m: &mut MarioState) {
    update_flying_pitch(m);
    update_flying_yaw(m);
    m.forward_vel -= 2.0 * ((m.face_angle[0] / 0x4000) as f32) + 0.1;
    m.forward_vel -= 0.5 * (1.0 - coss(m.angle_vel[1].cast_unsigned()));
    if m.forward_vel < 0.0 {
        m.forward_vel = 0.0
    }

    if m.forward_vel > 16.0 {
        m.face_angle[0] = m.face_angle[0].wrapping_add(((m.forward_vel - 32.0) * 6.0) as i16);
    } else if m.forward_vel > 4.0 {
        m.face_angle[0] = m.face_angle[0].wrapping_add(((m.forward_vel - 32.0) * 10.0) as i16);
    } else {
        m.face_angle[0] = m.face_angle[0].wrapping_sub(0x400);
    }
    m.face_angle[0] = m.face_angle[0].wrapping_add(m.angle_vel[0]);

    m.vel[0] = m.forward_vel * coss(m.face_angle[0].cast_unsigned()) * sins(m.face_angle[1].cast_unsigned());
    m.vel[1] = m.forward_vel * sins(m.face_angle[0].cast_unsigned());
    m.vel[2] = m.forward_vel * coss(m.face_angle[0].cast_unsigned()) * coss(m.face_angle[1].cast_unsigned());

    m.slide_vel_x = m.vel[0];
    m.slide_vel_z = m.vel[2];
}

pub fn update_air_without_turn(m: &mut MarioState) {
    let mut sideways_speed: f32 = 0.0;
    let mut intended_d_yaw: i16 = 0;
    let mut intended_mag: f32 = 0.0;
    let drag_threshold: f32 = 32.0; // not simulating long jump so 32.0f
    m.forward_vel = approach_value(m.forward_vel, 0.0f32, 0.35f32, 0.35f32);
    if m.controller.stick_x != 0.0 && m.controller.stick_y != 0.0 {
        intended_d_yaw = m.intended_yaw - m.face_angle[1];
        intended_mag = m.intended_mag / 32.0;
        m.forward_vel += intended_mag * coss(intended_d_yaw.cast_unsigned()) * 1.5;
        sideways_speed = intended_mag * sins(intended_d_yaw.cast_unsigned()) * 1.5;
    }

    // Uncapped air speed. Net positive when moving forward.
    if m.forward_vel > drag_threshold {
        m.forward_vel -= 1.0;
    }
    if m.forward_vel < 16.0 {
        m.forward_vel += 2.0;
    }
    m.slide_vel_x = m.forward_vel * sins(m.face_angle[1].cast_unsigned());
    m.slide_vel_z = m.forward_vel * coss(m.face_angle[1].cast_unsigned());

    m.slide_vel_x += sideways_speed * sins(m.face_angle[1].wrapping_add(0x4000).cast_unsigned());
    m.slide_vel_z += sideways_speed * coss(m.face_angle[1].wrapping_add(0x4000).cast_unsigned());

    m.vel[0] = m.slide_vel_x;
    m.vel[2] = m.slide_vel_z;

}

pub fn perform_air_step(m: &mut MarioState) {
    let mut intended_pos: [f32; 3] = [0.0, 0.0, 0.0];
    for _i in 0..3 {
        intended_pos[0] = m.pos[0] + (m.vel[0] / 4.0);
        intended_pos[1] = m.pos[1] + (m.vel[1] / 4.0);
        intended_pos[2] = m.pos[2] + (m.vel[2] / 4.0);
        m.pos = intended_pos;
    }
}
