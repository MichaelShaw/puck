extern crate cgmath;
#[macro_use]
extern crate puck_core;
extern crate alto;
extern crate lewton;
extern crate time;
extern crate notify;
extern crate rand;
extern crate image;

#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate rayon;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate multimap;

pub mod audio;
pub mod render;
pub mod app;
pub mod input;
pub mod dimensions;
pub mod camera;
pub mod resources;

pub use input::*;
pub use camera::*;
pub use dimensions::*;
pub use resources::*;

use std::io;
use std::path::PathBuf;



pub type PuckResult<T> = Result<T, PuckError>;

#[derive(Debug)]
pub enum PuckError {
    IO(io::Error),
    FileDoesntExist(PathBuf),
    PipelineError(gfx::PipelineStateError<String>),
    CombinedGFXError(gfx::CombinedError),
    ContextError(glutin::ContextError),
    NoTexture(),
    NoPipeline(),
    BufferCreationError(gfx::buffer::CreationError),
    TextureCreationError(gfx::texture::CreationError),
    ResourceViewError(gfx::ResourceViewError),
//    FontLoadError(FontLoadError),
    ImageError(image::ImageError),
    MustLoadTextureBeforeFont,
    NoFiles,
    MismatchingDimensions, // path buf, expectation
    RenderingPipelineIncomplete,
}

impl From<image::ImageError> for PuckError {
    fn from(err: image::ImageError) -> Self {
        PuckError::ImageError(err)
    }
}

impl From<io::Error> for PuckError {
    fn from(val: io::Error) -> PuckError {
        PuckError::IO(val)
    }
}

#[derive(Copy, Clone)]
pub struct RenderTick {
    pub n: u64,
    pub accu_alpha: f64, // percentage of a frame that has accumulated
    pub tick_rate: u64, // per second
}
