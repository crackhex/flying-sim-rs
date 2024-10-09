use std::cmp::Ordering;
use crate::includes::mario_state::MarioState;
use crate::includes::trig_table::{approach_value, coss, sins};

pub fn update_flying_yaw(m: &mut MarioState) {

    let target_yaw_vel: i16 = -(m.controller.stick_x * (m.forward_vel / 4.0)) as i16; // 0x4000
    match target_yaw_vel.cmp(&0i16) {
        Ordering::Greater => {
            if m.angle_vel[1] < 0 {
                m.angle_vel[1] += 0x40;
                if m.angle_vel[1] > 0x10 {
                    m.angle_vel[1] = 0x10;
                }
            } else {
                m.angle_vel[1] = approach_value(m.angle_vel[1] as i32, target_yaw_vel as i32, 0x10, 0x20) as i16; // why is this cast to i32 then i16
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
    let target_pitch_vel: i16 = -(m.controller.stick_y * (m.forward_vel / 5.0)) as i16; // 0x4000
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
    m.forward_vel -= 2.0 * (m.face_angle[0] as f32 / 16384.0_f32) + 0.1_f32; // 0x4000 = 16384
    m.forward_vel -= 0.5 * (1.0 - coss(m.angle_vel[1]));
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

    m.vel[0] = m.forward_vel * coss(m.face_angle[0]) * sins(m.face_angle[1]);
    m.vel[1] = m.forward_vel * sins(m.face_angle[0]);
    m.vel[2] = m.forward_vel * coss(m.face_angle[0]) * coss(m.face_angle[1]);

    m.slide_vel_x = m.vel[0];
    m.slide_vel_z = m.vel[2];
}

pub fn perform_air_step(m: &mut MarioState) {
    m.pos[0] += m.vel[0];
    m.pos[1] += m.vel[1];
    m.pos[2] += m.vel[2];
    // TODO: Make this iterate quarter steps if collision is enabled
    /*
    let mut intended_pos: [f32; 3] = [0.0, 0.0, 0.0];
    for _i in 0..4 {
        intended_pos[0] = m.pos[0] + (m.vel[0] / 4.0);
        intended_pos[1] = m.pos[1] + (m.vel[1] / 4.0);
        intended_pos[2] = m.pos[2] + (m.vel[2] / 4.0);
        m.pos = intended_pos;
    }*/
}
