use glutin;
use Dimensions;

pub fn get_dimensions(window: &glutin::GlWindow) -> Dimensions { // make this optional at some point
    Dimensions {
        pixels: window.get_inner_size_pixels().unwrap_or((100, 100)),
        points: window.get_inner_size_points().unwrap_or((100, 100)),
    }
}