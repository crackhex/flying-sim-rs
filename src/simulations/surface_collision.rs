use serde::{Deserialize, Serialize};

const LEVEL_BOUNDARY_MAX: i32 = 8192;
const CELL_SIZE: i32 = 0x400;
const CELL_HEIGHT_LIMIT: i32 = 20000;
const FLOOR_LOWER_LIMIT: i32 = -11000;

#[derive(Default, Serialize, Deserialize)]
pub struct SurfaceNormal {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Surface {
    type_: i16,                  // 0x00
    force: i16,                  // 0x02
    flags: i8,                   // 0x04
    room: i8,                    // 0x05
    lower_y: i16,                 // 0x06
    upper_y: i16,                 // 0x08
    vertex1: [i16; 3],           // 0x0A
    vertex2: [i16; 3],           // 0x10
    vertex3: [i16; 3],           // 0x16
    normal: SurfaceNormal,       // 0x1C
    origin_offset: f32,           // 0x28
    // object: *mut Object,      // 0x2C
}

pub enum SpacialPartition {
    Floors,
    Ceilings,
    Walls,
}

pub struct WallCollisionData {
    /*0x00*/ pub x: f32, pub y: f32, pub z: f32,
    /*0x0C*/ pub offset_y: f32,
    /*0x10*/ pub radius: f32,
    /*0x14*/ pub num_walls: i16,
    /*0x18*/ pub walls: [*mut Surface; 4],
}
