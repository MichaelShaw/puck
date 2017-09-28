pub mod gfx;
pub mod texture_array;
pub mod texture_region;
pub mod shader;

pub use self::shader::*;
pub use self::texture_array::*;
pub use self::texture_region::*;

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