use bitvec::prelude::BitArray;
use bitvec::view::BitViewSized;
use std::ascii::Char as AsciiChar;
use std::ops::{Range, Shr};
use thiserror::Error;

pub type Inputs = [Vec<Input>; 4];
pub type ByteVec = Vec<u8>;

pub struct M64Header {
    pub signature: [u8; 4],             //0x00 4 bytes
    pub version: u32,                   //0x04
    pub uid: i32,                       //0x08
    pub vi_count: u32,                  //0x0C
    pub rerecord_count: u32,            //0x10
    pub vi_per_second: u8,              //0x14
    pub controller_count: u8,           //0x15
    pub num_samples: u32,               //0x18
    pub movie_start_type: u16,          //0x1C
    pub controller_flags: u32,          //0x20
    pub internal_name: [AsciiChar; 32], //0xC4 32 bytes
    pub crc32: u32,                     //0xE4
    pub country_code: u16,              //0xE8
    pub video_plugin: [AsciiChar; 64],  //0x122 64 bytes
    pub sound_plugin: [AsciiChar; 64],  //0x162 64 bytes
    pub input_plugin: [AsciiChar; 64],  //0x1A2 64 bytes
    pub rsp_plugin: [AsciiChar; 64],    //0x1E2 64 bytes
    pub author: [AsciiChar; 222],       //0x222 220 bytes
    pub movie_desc: [AsciiChar; 256],   //0x300 256 bytes
}

pub struct M64File {
    pub header: M64Header,
    pub inputs: Inputs,
}

#[derive(Debug, Error)]
pub enum M64Error {
    #[error("File cannot be read")]
    File(#[from] std::io::Error),
    #[error("Error Parsing data")]
    ParsingError,
    #[error("Header is too short")]
    HeaderIncomplete,
}

#[derive(Clone, Default)]
pub struct Input {
    pub r_dpad: bool,
    pub l_dpad: bool,
    pub d_dpad: bool,
    pub u_dpad: bool,
    pub start: bool,
    pub z_trig: bool,
    pub b_button: bool,
    pub a_button: bool,
    pub c_right: bool,
    pub c_left: bool,
    pub c_down: bool,
    pub c_up: bool,
    pub r_trig: bool,
    pub l_trig: bool,
    pub x: i8,
    pub y: i8,
}

pub fn active_controllers(controller_flags: u32) -> Result<Vec<usize>, M64Error> {
    // Returns a vector with the indices of the active controllers,
    // e.g., if controller 1, 2, and 4 are enabled, it will return [1, 2, 4]
    let controllers: BitArray<u32> = controller_flags.into_bitarray();
    let active_controllers: Vec<usize> = (0..4).filter(|&i| controllers[i]).collect::<Vec<_>>();

    (!active_controllers.is_empty())
        .then_some(active_controllers)
        .ok_or(M64Error::ParsingError)
}
impl Input {
    fn parse(input_bytes: &ByteVec, controller_flags: u8) -> Result<Inputs, M64Error> {
        let mut inputs: Inputs = Inputs::default();
        let active_controllers = active_controllers(controller_flags as u32)?;
        for i in (0..input_bytes.len()).step_by(4) {
            let input = u32::from_le_bytes(input_bytes[i..i + 4].try_into().unwrap());
            let current_controller = active_controllers[(i / 4) % active_controllers.len()];
            inputs[current_controller].push(Self {
                r_dpad: (input & 0x01) != 0,
                l_dpad: (input & 0x02) != 0,
                d_dpad: (input & 0x04) != 0,
                u_dpad: (input & 0x08) != 0,
                start: (input & 0x10) != 0,
                z_trig: (input & 0x20) != 0,
                b_button: (input & 0x40) != 0,
                a_button: (input & 0x80) != 0,
                c_right: (input & 0x100) != 0,
                c_left: (input & 0x200) != 0,
                c_down: (input & 0x400) != 0,
                c_up: (input & 0x800) != 0,
                r_trig: (input & 0x1000) != 0,
                l_trig: (input & 0x2000) != 0,
                x: input.shr(16) as i8,
                y: input.shr(24) as i8,
            });
        }
        Ok(inputs)
    }
}

impl M64Header {
    fn new() -> M64Header {
        M64Header {
            signature: [0x4D, 0x36, 0x34, 0x1A],
            version: 0x03,
            uid: 0,
            vi_count: 0,
            rerecord_count: 0,
            vi_per_second: 0,
            controller_count: 0,
            num_samples: 0,
            movie_start_type: 0,
            controller_flags: 0,
            internal_name: [0_u8.as_ascii().unwrap(); 32],
            crc32: 0,
            country_code: 0,
            video_plugin: [0_u8.as_ascii().unwrap(); 64],
            sound_plugin: [0_u8.as_ascii().unwrap(); 64],
            input_plugin: [0_u8.as_ascii().unwrap(); 64],
            rsp_plugin: [0_u8.as_ascii().unwrap(); 64],
            author: [0_u8.as_ascii().unwrap(); 222],
            movie_desc: [0_u8.as_ascii().unwrap(); 256],
        }
    }
    pub fn from_bytes(buf: &ByteVec) -> Result<M64Header, M64Error> {
        if buf.len() < 0x400 {
            return Err(M64Error::HeaderIncomplete);
        }
        let m64 = M64Header {
            signature: buf[0x0..0x4].try_into().unwrap(),
            version: u32::from_le_bytes(buf[0x4..0x8].try_into().unwrap()),
            uid: i32::from_le_bytes(buf[0x8..0xC].try_into().unwrap()),
            vi_count: u32::from_le_bytes(buf[0xC..0x10].try_into().unwrap()),
            rerecord_count: u32::from_le_bytes(buf[0x10..0x14].try_into().unwrap()),
            vi_per_second: buf[0x14],
            controller_count: buf[0x15],
            num_samples: u32::from_le_bytes(buf[0x18..0x1C].try_into().unwrap()),
            movie_start_type: u16::from_le_bytes(buf[0x1C..0x1E].try_into().unwrap()),
            controller_flags: u32::from_le_bytes(buf[0x20..0x24].try_into().unwrap()),
            internal_name: *<&[u8] as TryInto<[u8; 32]>>::try_into(&buf[0xC4..0xE4])
                .unwrap()
                .as_ascii()
                .unwrap(),
            crc32: u32::from_le_bytes(buf[0xE4..0xE8].try_into().unwrap()),
            country_code: u16::from_le_bytes(buf[0xE8..0xEA].try_into().unwrap()),
            video_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buf[0x122..0x162])
                .unwrap()
                .as_ascii()
                .unwrap(),
            sound_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buf[0x162..0x1A2])
                .unwrap()
                .as_ascii()
                .unwrap(),
            input_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buf[0x1A2..0x1E2])
                .unwrap()
                .as_ascii()
                .unwrap(),
            rsp_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buf[0x1E2..0x222])
                .unwrap()
                .as_ascii()
                .unwrap(),
            author: *<&[u8] as TryInto<[u8; 222]>>::try_into(&buf[0x222..0x300])
                .unwrap()
                .as_ascii()
                .unwrap(),
            movie_desc: *<&[u8] as TryInto<[u8; 256]>>::try_into(&buf[0x300..0x400])
                .unwrap()
                .as_ascii()
                .unwrap(),
        };
        Ok(m64)
    }
}
impl M64File {
    pub fn new() -> Self {
        Self {
            header: M64Header::new(),
            inputs: Inputs::default(),
        }
    }
    pub fn samples_to_bytes(&self, active_controllers: &[usize]) -> Result<ByteVec, M64Error> {
        let size = self.inputs[0].len()
            + self.inputs[1].len()
            + self.inputs[2].len()
            + self.inputs[3].len();
        let mut input_bytes: ByteVec = vec![0; size * 4];
        for i in 0..size {
            let current_controller = active_controllers[i % active_controllers.len()];
            let frame = i.div_floor(active_controllers.len());
            let input = &self.inputs[current_controller][frame];
            let mut input_byte: u32 = 0;
            input_byte |= input.r_dpad as u32;
            input_byte |= (input.l_dpad as u32) << 1;
            input_byte |= (input.d_dpad as u32) << 2;
            input_byte |= (input.u_dpad as u32) << 3;
            input_byte |= (input.start as u32) << 4;
            input_byte |= (input.z_trig as u32) << 5;
            input_byte |= (input.b_button as u32) << 6;
            input_byte |= (input.a_button as u32) << 7;
            input_byte |= (input.c_right as u32) << 8;
            input_byte |= (input.c_left as u32) << 9;
            input_byte |= (input.c_down as u32) << 10;
            input_byte |= (input.c_up as u32) << 11;
            input_byte |= (input.r_trig as u32) << 12;
            input_byte |= (input.l_trig as u32) << 13;
            input_byte |= ((input.x as u8) as u32) << 16;
            input_byte |= ((input.y as u8) as u32) << 24;
            input_bytes[i * 4..i * 4 + 4].copy_from_slice(&input_byte.to_le_bytes());
        }
        Ok(input_bytes)
    }

    pub fn to_bytes(&self) -> Result<ByteVec, M64Error> {
        let active_controllers = active_controllers(self.header.controller_flags)?;
        let mut sample_bytes: ByteVec = M64File::samples_to_bytes(self, &active_controllers)?;
        let mut buffer: ByteVec = vec![0; 0x400 + sample_bytes.len()];
        buffer[0x0..0x4].copy_from_slice(&self.header.signature);
        buffer[0x4..0x8].copy_from_slice(&self.header.version.to_le_bytes());
        buffer[0x8..0xC].copy_from_slice(&self.header.uid.to_le_bytes());
        buffer[0xC..0x10].copy_from_slice(&self.header.vi_count.to_le_bytes());
        buffer[0x10..0x14].copy_from_slice(&self.header.rerecord_count.to_le_bytes());
        buffer[0x14] = self.header.vi_per_second;
        buffer[0x15] = self.header.controller_count;
        buffer[0x18..0x1C].copy_from_slice(&self.header.num_samples.to_le_bytes());
        buffer[0x1C..0x1E].copy_from_slice(&self.header.movie_start_type.to_le_bytes());
        buffer[0x20..0x24].copy_from_slice(&self.header.controller_flags.to_le_bytes());
        buffer[0xC4..0xE4].copy_from_slice(self.header.internal_name.as_bytes());
        buffer[0xE4..0xE8].copy_from_slice(&self.header.crc32.to_le_bytes());
        buffer[0xE8..0xEA].copy_from_slice(&self.header.country_code.to_le_bytes());
        buffer[0x122..0x162].copy_from_slice(self.header.video_plugin.as_bytes());
        buffer[0x162..0x1A2].copy_from_slice(self.header.sound_plugin.as_bytes());
        buffer[0x1A2..0x1E2].copy_from_slice(self.header.input_plugin.as_bytes());
        buffer[0x1E2..0x222].copy_from_slice(self.header.rsp_plugin.as_bytes());
        buffer[0x222..0x300].copy_from_slice(self.header.author.as_bytes());
        buffer[0x300..0x400].copy_from_slice(self.header.movie_desc.as_bytes());
        buffer[0x400..].copy_from_slice(&sample_bytes);
        Ok(buffer)
    }

    pub fn remove_inputs(&mut self, range: &Range<usize>) -> Result<&mut M64File, M64Error> {
        let active_controllers = active_controllers(self.header.controller_flags)?;
        for i in 0..active_controllers.len() {
            self.inputs[active_controllers[i]].drain(&range.start..&range.end);
        }
        Ok(self)
    }
    pub fn add_inputs(&mut self, range: &Range<usize>) -> Result<&mut M64File, M64Error> {
        let active_controllers = active_controllers(self.header.controller_flags)?;
        for i in 0..active_controllers.len() {
            let inputs = &mut self.inputs[active_controllers[i]];
            let end = inputs.split_off(range.start);
            inputs.extend_from_slice(&vec![Input::default(); range.end - range.start][..]);
            inputs.extend_from_slice(&end);
        }
        Ok(self)
    }
}
