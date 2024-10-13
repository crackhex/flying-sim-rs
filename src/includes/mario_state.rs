use serde::{Deserialize, Serialize};
use crate::simulations::flying_sim::{update_flying, perform_air_step};
use crate::simulations::object_collision::{Object, Interact};

#[derive(Default)]
pub struct ControllerRaw {
    x: i8,
    y: i8,
}

impl From<i16> for ControllerRaw {
    fn from(value: i16) -> Self {
        Self {
            x: ((value >> 8) & 0xFF) as i8,
            y: (value & 0xFF) as i8,
        }
    }
}

impl From<[i8; 2]> for ControllerRaw {
    fn from(values: [i8; 2]) -> Self {
        Self {
            x: values[0],
            y: values[1],
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Controller {
    pub stick_x: f32,
    pub stick_y: f32,
    stick_mag: f32,
}

impl Controller {
    pub fn update_joystick<T: Into<ControllerRaw>>(&mut self, raw_stick: T) {
        let input: ControllerRaw = raw_stick.into();
        self.stick_x = 0.0;
        self.stick_y = 0.0;
        if input.x <= -8 {
            self.stick_x = (input.x + 6) as f32;
        }
        if input.x >= 8 {
            self.stick_x = (input.x - 6) as f32;
        }
        if input.y <= -8 {
            self.stick_y = (input.x + 6) as f32;
        }
        if input.y >= 8 {
            self.stick_y = (input.y - 6) as f32;
        }

        self.stick_mag = (self.stick_x * self.stick_x + self.stick_y * self.stick_y).sqrt();
        if self.stick_mag > 64.0 {
            self.stick_x *= 64.0 / self.stick_mag;
            self.stick_y *= 64.0 / self.stick_mag;
            self.stick_mag = 64.0;
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct MarioState {
    pub input: u16,                 // 0x02
    pub flags: u32,                 // 0x04
    pub action: u32,                // 0x0C
    pub prev_action: u32,           // 0x10
    action_state: u32,              // 0x18
    action_timer: u16,              // 0x1A
    action_arg: u32,                // 0x1C
    pub intended_mag: f32,          // 0x1E
    pub intended_yaw: i16,          // 0x22
    pub frames_since_a: u8,         // 0x28
    frames_since_b: u8,             // 0x29
    pub face_angle: [i16; 3],       // 0x2C
    pub angle_vel: [i16; 3],        // 0x32
    slide_yaw: i16,                 // 0x38
    pub pos: [f32; 3],              // 0x3C
    pub vel: [f32; 3],              // 0x48
    pub forward_vel: f32,           // 0x54
    pub slide_vel_x: f32,           // 0x58
    pub slide_vel_z: f32,           // 0x5C
    ceil_height: f32,               // 0x6C
    floor_height: f32,              // 0x70
    floor_angle: i16,               // 0x74
    pub controller: Controller,     // 0x78
    num_coins: i16,                 // 0xA8
}

impl MarioState {
    pub fn update_flying<T: Into<ControllerRaw>>(&mut self, inputs: T) {
        self.controller.update_joystick(inputs);
        update_flying(self);
        perform_air_step(self);
        println!("{:?}", self.pos)
    }

    pub fn collect_closest_object(&self, mut obj_list: Vec<Object>) -> Vec<Object> {
        let mut smallest_dist: f32 = 1000.0;
        let mut obj_index: usize = 0;
        for (i, obj) in obj_list.iter().enumerate() {
            let dist = obj.dist_to_mario(self);
            if dist < smallest_dist {
                smallest_dist = dist;
                obj_index = i;
            }
        }
        obj_list.remove(obj_index);
        obj_list
    }
}