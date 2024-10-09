use crate::includes::flying_sim::{perform_air_step, update_flying};

pub struct Controller {
    raw_stick: RawStick,
    pub(crate) stick_x: f32,
    pub(crate) stick_y: f32,
    stick_mag: f32,               // 0x0C
    button_down: u16,             // 0x10
    button_pressed: u16,          // 0x12
}

#[derive(Default)]
struct RawStick {
    x: i16,
    y: i16,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            raw_stick: RawStick::default(),
            stick_x: 0.0,
            stick_y: 0.0,
            stick_mag: 0.0,
            button_down: 0,
            button_pressed: 0,
        }
    }
    pub fn update_input(&mut self, raw_x: i8, raw_y: i8) {
        self.raw_stick.x = raw_x as i16;
        self.raw_stick.y = raw_y as i16;
        self.stick_x = 0.0;
        self.stick_y = 0.0;
        if self.raw_stick.x <= -8 {
            self.stick_x = (self.raw_stick.x + 6) as f32;
        }
        if self.raw_stick.x >= 8 {
            self.stick_x = (self.raw_stick.x - 6) as f32;
        }
        if self.raw_stick.y <= -8 {
            self.stick_y = (self.raw_stick.y + 6) as f32;
        }
        if self.raw_stick.y >= 8 {
            self.stick_y = (self.raw_stick.y - 6) as f32;
        }

        self.stick_mag = (self.stick_x * self.stick_x + self.stick_y * self.stick_y).sqrt();
        if self.stick_mag > 64.0 {
            self.stick_x *= 64.0 / self.stick_mag;
            self.stick_y *= 64.0 / self.stick_mag;
            self.stick_mag = 64.0;
        }
    }
}


pub struct MarioState {
    pub input: u16,                   // 0x02
    pub flags: u32,                   // 0x04
    pub action: u32,                  // 0x0C
    pub prev_action: u32,              // 0x10
    action_state: u32,             // 0x18
    action_timer: u16,             // 0x1A
    action_arg: u32,               // 0x1C
    pub intended_mag: f32,             // 0x1E
    pub intended_yaw: i16,             // 0x22
    pub frames_since_a: u8,             // 0x28
    frames_since_b: u8,             // 0x29
    pub face_angle: [i16; 3],          // 0x2C
    pub angle_vel: [i16; 3],           // 0x32
    slide_yaw: i16,                // 0x38
    pub pos: [f32; 3],                // 0x3C
    pub vel: [f32; 3],                // 0x48
    pub forward_vel: f32,              // 0x54
    pub slide_vel_x: f32,               // 0x58
    pub slide_vel_z: f32,               // 0x5C
    ceil_height: f32,              // 0x6C
    floor_height: f32,             // 0x70
    floor_angle: i16,              // 0x74
    pub controller: Controller,       // 0x78
    num_coins: i16,                // 0xA8
}

impl MarioState {
    pub fn new() -> MarioState {
        
        MarioState {
            input: 0,
            flags: 0,
            action: 0,
            prev_action: 0,
            action_state: 0,
            action_timer: 0,
            action_arg: 0,
            intended_mag: 0.0,
            intended_yaw: 0,
            frames_since_a: 0,
            frames_since_b: 0,
            face_angle: [0, 0, 0],
            angle_vel: [0, 0, 0],
            slide_yaw: 0,
            pos: [0.0, 0.0, 0.0],
            vel: [0.0, 0.0, 0.0],
            forward_vel: 0.0,
            slide_vel_x: 0.0,
            slide_vel_z: 0.0,
            ceil_height: 0.0,
            floor_height: 0.0,
            floor_angle: 0,
            controller: Controller::new(),
            num_coins: 0,
        }
    }
    pub fn update_state(&mut self, raw_x: i8, raw_y: i8) {
        self.controller.update_input(raw_x, raw_y);
        println!("{}, {}", self.controller.stick_x, self.controller.stick_y);
        update_flying(self);
        perform_air_step(self);
        println!("{:?}", self.pos)
    }
}