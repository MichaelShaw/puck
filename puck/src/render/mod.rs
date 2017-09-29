pub mod gfx;
pub mod texture_array;
pub mod texture_region;
pub mod shader;
pub mod quads;

pub use self::shader::*;
pub use self::texture_array::*;
pub use self::texture_region::*;
pub use self::quads::*;

use image::Rgba;
use puck_core::color::Color;

pub fn as_rgba8(color:Color) -> Rgba<u8> {
    Rgba { data: color.raw() }
}

pub fn down_size_m4(arr: [[f64; 4];4]) -> [[f32; 4]; 4] {
    let mut out : [[f32; 4]; 4] = [[0.0; 4]; 4];
    for a in 0..4 {
        for b in 0..4 {
            out[a][b] = arr[a][b] as f32
        }
    }

    out
}

pub type Vertex = self::gfx::Vertex;
pub type BufferData = Vec<Vertex>;
pub type Transform = [[f32; 4]; 4];

#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    pub transform : Transform,
    pub color: Color,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Blend {
    None,
    Add,
    Alpha,
}