const LEVEL_BOUNDARY_MAX: i32 = 8192;
const CELL_SIZE: i32 = 0x400;
const CELL_HEIGHT_LIMIT: i32 = 20000;
const FLOOR_LOWER_LIMIT: i32 = -11000;

pub struct SurfaceNormal {
    x: f32,
    y: f32,
    z: f32,
}
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

pub struct SurfaceNode {
    next: *mut SurfaceNode,
    surface: *mut Surface,
}

pub enum SpacialPartition {
    Floors,
    Ceilings,
    Walls,
}
pub type SpacialPartitionCell = [SurfaceNode; 3];

pub struct GraphNode {
    type_: i16,                  // 0x00
    flags: i16,                  // 0x02
    prev: *mut GraphNode,            // 0x04
    next: *mut GraphNode,            // 0x08
    parent: *mut GraphNode,          // 0x0C
    children: *mut GraphNode,        // 0x10
}

pub struct GraphNodeObject {
    node: GraphNode,
    shared_child: *mut GraphNode,     // 0x14
    area_index: i8,               // 0x18
    active_area_index: i8,         // 0x19
    angle: [i16; 3],             // 0x1A
    pos: [f32; 3],               // 0x20
    scale: [f32; 3],             // 0x2C
    // animInfo: AnimInfo,       // 0x38
    // unk4C: *SpawnInfo,        // 0x4C
    throw_matrix: *mut [[f32; 4]; 4], // 0x50  // Not sure if this is right
    camera_to_object: [f32; 3],    // 0x54
}


pub struct WallCollisionData {
    /*0x00*/ pub x: f32, pub y: f32, pub z: f32,
    /*0x0C*/ pub offset_y: f32,
    /*0x10*/ pub radius: f32,
    /*0x14*/ pub num_walls: i16,
    /*0x18*/ pub walls: [*mut Surface; 4],
}

fn find_wall_collisions(surface_node: &SurfaceNode, data: &WallCollisionData) {
    let mut radius = data.radius;
    let x = data.x;
    let y = data.y + data.offset_y;
    let z = data.z;
    let num_cols = 0;

    if radius > 200.0 {
        radius = 200.0;
    }


}