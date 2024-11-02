use crate::simulations::flying_sim::{perform_air_step, update_flying};
use crate::simulations::object_collision::{Interact, Targets};
use crate::utils::file_handling::InputFile;
use serde::{Deserialize, Serialize};

pub const fn pack_input(x: i8, y: i8) -> i16 {
    ((x as i16) << 8) + ((y as i16) & 255)
}
pub const fn pack_input_u8(input: [u8; 2]) -> i16 {
    ((input[0] as i16) << 8) + ((input[1] as i16) & 255)
}

pub const fn unpack_input_i8(input: i16) -> [i8; 2] {
    [((input >> 8i16) & 0xFF) as i8, (input & 0xFF) as i8]
}
#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone)]
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

#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone)]
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
    pub fn update_flying(&mut self, input: &i16) {
        self.controller.update_joystick(input);
        update_flying(self);
        perform_air_step(self);
        //println!("{:?}", self.pos)
    }
    pub fn hit_closest_target(&self, t: &mut Targets) {
        let mut min_dist = f32::MAX;
        let mut index: usize = 0;
        t.cylinder.iter_mut().enumerate().for_each(|(i, obj)| {
            if obj.is_active() {
                let dist = obj.horizontal_dist_to_mario(self.pos);
                if dist < obj.radius
                    && dist < min_dist
                    && self.pos[1] >= (obj.pos[1] - 160.0f32)
                    && self.pos[1] <= (obj.pos[1] + 64.0f32)
                {
                    min_dist = dist;
                    index = i + 1;
                }
            }
        });
        if index > 0 {
            t.cylinder[index - 1].active = false;
        }
        t.cuboid.iter_mut().for_each(|hitbox| {
            if hitbox.is_active() && hitbox.is_mario_in_bounds(self.pos) {
                hitbox.active = false;
            }
        })
    }
    pub fn hit_goal(&self, t: impl Interact) -> bool {
        if t.is_mario_in_bounds(self.pos) {
            //println!("{:?}", t.horizontal_dist_to_mario(self.pos));
            return true;
        }
        false
    }
    fn is_mario_in_bounds(&self, mario_pos: [f32; 3]) -> bool {
        if mario_pos[1] >= self.pos[1] - 160.0f32 && mario_pos[1] <= self.pos[1] + 64.0f32 {
            return true;
        }
        false
    }
}
pub fn simulate_inputs(m: &mut MarioState, targets: &mut Targets, inputs: &[i16]) {
    inputs.iter().for_each(|input| {
        m.update_flying(input);
        m.hit_closest_target(targets);
    });
}
pub fn simulate(input_file: &mut InputFile) {
    let m = &mut input_file.initial_state;
    let x = &*input_file.inputs;
    let mut targets = &mut input_file.targets;
    println!("{:?}", targets);
    for (i, input) in x.iter().enumerate() {
        m.update_flying(input);
        m.hit_closest_target(targets);
        if i < x.len() - 1 {
            println!(
                " {:?} {:?} {:?} {:?} {:?}",
                m.pos,
                m.face_angle[0],
                ((input >> 8i16) & 0xFF) as i8,
                (input & 0xFF) as i8,
                m.angle_vel[0]
            );
        }
    }
    println! {"{:?}", targets}
    //simulate_inputs(mario, x.into());
}
