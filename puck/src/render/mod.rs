pub mod dimensions;
pub mod input;
pub mod camera;

use image::Rgba;
use puck_core::color::Color;

// color conversion

pub fn as_rgba8(color:Color) -> Rgba<u8> {
    Rgba { data: color.raw() }
}