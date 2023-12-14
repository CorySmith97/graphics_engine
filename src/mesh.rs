pub struct Mesh<'a> {
    pub verticies: &'a [Vertex],
    pub indices: &'a [u16],
}

impl<'a> Mesh<'a> {
    pub fn new_cube() -> Self {
        #[rustfmt::skip]
        const VERTICES: &[Vertex] = &[
            // pos                                color              uvs
            Vertex { x: -1.0,  y: -1.0, z: -1.0,  color: 0xFFFFFFFF, u:     0, v:     0 },
            Vertex { x:  1.0,  y: -1.0, z: -1.0,  color: 0xFF0000FF, u: 32767, v:     0 },
            Vertex { x:  1.0,  y:  1.0, z: -1.0,  color: 0xFF0000FF, u: 32767, v: 32767 },
            Vertex { x: -1.0,  y:  1.0, z: -1.0,  color: 0xFF0000FF, u:     0, v: 32767 },

            Vertex { x: -1.0,  y: -1.0, z:  1.0,  color: 0xFF00FF00, u:     0, v:     0 },
            Vertex { x:  1.0,  y: -1.0, z:  1.0,  color: 0xFF00FF00, u: 32767, v:     0 },
            Vertex { x:  1.0,  y:  1.0, z:  1.0,  color: 0xFF00FF00, u: 32767, v: 32767 },
            Vertex { x: -1.0,  y:  1.0, z:  1.0,  color: 0xFF00FF00, u:     0, v: 32767 },

            Vertex { x: -1.0,  y: -1.0, z: -1.0,  color: 0xFFFF0000, u:     0, v:     0 },
            Vertex { x: -1.0,  y:  1.0, z: -1.0,  color: 0xFFFF0000, u: 32767, v:     0 },
            Vertex { x: -1.0,  y:  1.0, z:  1.0,  color: 0xFFFF0000, u: 32767, v: 32767 },
            Vertex { x: -1.0,  y: -1.0, z:  1.0,  color: 0xFFFF0000, u:     0, v: 32767 },

            Vertex { x:  1.0,  y: -1.0, z: -1.0,  color: 0xFFFF007F, u:     0, v:     0 },
            Vertex { x:  1.0,  y:  1.0, z: -1.0,  color: 0xFFFF007F, u: 32767, v:     0 },
            Vertex { x:  1.0,  y:  1.0, z:  1.0,  color: 0xFFFF007F, u: 32767, v: 32767 },
            Vertex { x:  1.0,  y: -1.0, z:  1.0,  color: 0xFFFF007F, u:     0, v: 32767 },

            Vertex { x: -1.0,  y: -1.0, z: -1.0,  color: 0xFFFF7F00, u:     0, v:     0 },
            Vertex { x: -1.0,  y: -1.0, z:  1.0,  color: 0xFFFF7F00, u: 32767, v:     0 },
            Vertex { x:  1.0,  y: -1.0, z:  1.0,  color: 0xFFFF7F00, u: 32767, v: 32767 },
            Vertex { x:  1.0,  y: -1.0, z: -1.0,  color: 0xFFFF7F00, u:     0, v: 32767 },

            Vertex { x: -1.0,  y:  1.0, z: -1.0,  color: 0xFF007FFF, u:     0, v:     0 },
            Vertex { x: -1.0,  y:  1.0, z:  1.0,  color: 0xFF007FFF, u: 32767, v:     0 },
            Vertex { x:  1.0,  y:  1.0, z:  1.0,  color: 0xFF007FFF, u: 32767, v: 32767 },
            Vertex { x:  1.0,  y:  1.0, z: -1.0,  color: 0xFF007FFF, u:     0, v: 32767 },
        ];

        #[rustfmt::skip]
        const INDICES: &[u16] = &[
            0, 1, 2,  0, 2, 3,
            6, 5, 4,  7, 6, 4,
            8, 9, 10,  8, 10, 11,
            14, 13, 12,  15, 14, 12,
            16, 17, 18,  16, 18, 19,
            22, 21, 20,  23, 22, 20,
        ];

        Self {
            verticies: VERTICES,
            indices: INDICES,
        }
    }
}

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub color: u32,
    pub u: u16,
    pub v: u16,
}
