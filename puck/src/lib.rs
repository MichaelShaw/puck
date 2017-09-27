extern crate cgmath;
extern crate puck_core;
extern crate alto;
extern crate lewton;
extern crate time;
extern crate notify;
extern crate rand;
extern crate image;
#[macro_use]
extern crate gfx;

extern crate glutin;

pub mod audio;
pub mod render;
pub mod app;
pub mod input;
pub mod dimensions;

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

