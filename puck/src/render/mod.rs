pub mod camera;
pub mod gfx;
pub mod texture_array;
pub mod texture_region;
pub mod shader;

use image::Rgba;
use puck_core::color::Color;

pub fn as_rgba8(color:Color) -> Rgba<u8> {
    Rgba { data: color.raw() }
}