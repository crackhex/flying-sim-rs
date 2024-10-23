use crate::simulations::flying_sim::{perform_air_step, update_flying};
use crate::simulations::object_collision::{Interact, Targets};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn pack_input(x: i8, y: i8) -> i16 {
    ((x as i16) << 8) + ((y as i16) & 255)
}
pub fn pack_input_u8(input: [u8; 2]) -> i16 {
    ((input[0] as i16) << 8) + ((input[1] as i16) & 255)
}
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Controller {
    pub stick_x: f32,
    pub stick_y: f32,
    stick_mag: f32,
}

impl Controller {
    pub fn update_joystick(&mut self, joystick: &i16) {
        let x = ((joystick >> 8i16) & 0xFF) as i8;
        let y = (joystick & 0xFF) as i8;
        self.stick_x = 0.0;
        self.stick_y = 0.0;
        if x <= -8 {
            self.stick_x = (x + 6) as f32;
        }
        if x >= 8 {
            self.stick_x = (x - 6) as f32;
        }
        if y <= -8 {
            self.stick_y = (y + 6) as f32;
        }
        if y >= 8 {
            self.stick_y = (y - 6) as f32;
        }

        self.stick_mag = (self.stick_x * self.stick_x + self.stick_y * self.stick_y).sqrt();
        if self.stick_mag > 64.0 {
            self.stick_x *= 64.0 / self.stick_mag;
            self.stick_y *= 64.0 / self.stick_mag;
            self.stick_mag = 64.0;
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct MarioState {
    pub input: u16,             // 0x02
    pub controller: Controller, // 0x78
    pub action: u32,            // 0x0C
    pub intended_mag: f32,      // 0x1E
    pub intended_yaw: i16,      // 0x22
    pub face_angle: [i16; 3],   // 0x2C
    pub angle_vel: [i16; 3],    // 0x32
    pub pos: [f32; 3],          // 0x3C
    pub vel: [f32; 3],          // 0x48
    pub forward_vel: f32,       // 0x54
    pub slide_vel_x: f32,       // 0x58
    pub slide_vel_z: f32,       // 0x5C
    ceil_height: f32,           // 0x6C
    floor_height: f32,          // 0x70
    floor_angle: i16,           // 0x74
    num_coins: i16,             // 0xA8
}

impl MarioState {
    pub fn update_flying(&mut self, inputs: &i16) {
        self.controller.update_joystick(inputs);
        update_flying(self);
        perform_air_step(self);
        //println!("{:?}", self.pos)
    }

    pub fn collect_closest_object<T: Interact>(&self, t: &mut Targets<T>) {
        let mut smallest_dist: f32 = 1000.0;
        let mut obj_index: usize = 0;
        for (i, obj) in t.data.iter().enumerate() {
            if obj.is_active() {
                let dist = obj.horizontal_dist_to_mario(&self.pos);
                if dist < smallest_dist {
                    smallest_dist = dist;
                    obj_index = i;
                }
            }
            //t.data[obj_index] = false;
        }
    }
}

pub fn simulate_inputs(m: &mut MarioState, inputs: Arc<[i16]>) {
    for input in inputs.iter() {
        m.update_flying(input);
    }
}
