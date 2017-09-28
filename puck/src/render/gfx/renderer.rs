use gfx;
use glutin;
use gfx_device_gl;

use gfx::format::{Srgba8, Rgba8};
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_window_glutin;

use gfx::texture::ImageInfoCommon;
use gfx::format::R8_G8_B8_A8;

use super::{Vertex, ColorFormat, DepthFormat, GeometryBuffer, Locals};
use super::{pipe_blend, pipe_opaque, get_dimensions};

use puck_core::Color;
use {input, PuckError, PuckResult};
//use render::{Uniforms, Blend, TextureRegion, GeometryTesselator};
use {Dimensions, Input};
use glutin::GlContext;
use camera::ui_projection;
use render::{down_size_m4, TextureArrayDimensions};
use FileResources;
use std::path::PathBuf;

use image::DynamicImage;

use puck_core::HashMap;

use cgmath::{Vector2, vec3};

pub struct OpaquePipeline<R> where R : gfx::Resources {
    pub pipeline: gfx::PipelineState<R, pipe_opaque::Meta>,
    pub data : Option<pipe_opaque::Data<R>>,
}

pub struct BlendPipeline<R> where R : gfx::Resources {
    pub pipeline: gfx::PipelineState<R, pipe_blend::Meta>,
    pub data : Option<pipe_blend::Data<R>>,
}

pub struct Pipelines<R> where R : gfx::Resources {
    pub opaque: OpaquePipeline<R>,
    pub blend: BlendPipeline<R>,
}

pub struct Renderer<R, C, F, D> where R : gfx::Resources,
                                      C : gfx::CommandBuffer<R>,
                                      F : gfx::Factory<R>,
                                      D : gfx::Device {
    pub file_resources: FileResources,

    pub window: glutin::GlWindow, // opengl
    pub events_loop: glutin::EventsLoop, // opengl

    pub device: D,
    pub factory: F,

    pub screen_colour_target: gfx::handle::RenderTargetView<R, ColorFormat>,
    pub screen_depth_target: gfx::handle::DepthStencilView<R, DepthFormat>,
    pub encoder: gfx::Encoder<R, C>,

    // what about raw texture representation? for blitting to ui
    pub texture: Option<(gfx::handle::Texture<R, gfx::format::R8_G8_B8_A8>, gfx::handle::ShaderResourceView<R, [f32; 4]>)>,

    pub sampler: gfx::handle::Sampler<R>,

    pub pipelines: Option<Pipelines<R>>,

    pub dimensions: Dimensions,
    pub input: Input,
}

impl<F> Renderer<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer, F, gfx_device_gl::Device> where F : gfx::Factory<gfx_device_gl::Resources> {
    pub fn load_resources(&mut self, reload_texture: bool, reload_program: bool) -> bool {
        if reload_program || self.pipelines.is_none() {
            //            println!("LOAD PIPELINES");
            let pipeline_load_result = self.file_resources.shader_pair.load().and_then( |shader_data| {
                let opaque_pso = self.factory.create_pipeline_simple(
                    &shader_data.vertex_data,
                    &shader_data.fragment_data,
                    pipe_opaque::new()
                ).map_err(PuckError::PipelineError)?;

                let blend_pso = self.factory.create_pipeline_simple(
                    &shader_data.vertex_data,
                    &shader_data.fragment_data,
                    pipe_blend::new()
                ).map_err(PuckError::PipelineError)?;

                Ok(Pipelines {
                    opaque: OpaquePipeline {
                        pipeline: opaque_pso,
                        data: None,
                    },
                    blend: BlendPipeline {
                        pipeline: blend_pso,
                        data: None,
                    },
                })
            });

            match pipeline_load_result {
                Ok(p) => self.pipelines = Some(p),
                Err(e) => println!("pipeline load error -> {:?}", e),
            }
        }

        if reload_texture || self.texture.is_none() {
            //            println!("LOAD TEXTURES");
            let texture_load_result = self.file_resources.texture_directory.load().and_then(|texture_array_data| {
                let images_raw : Vec<_> = texture_array_data.images.iter().map(|img| {
                    let dyn_image = DynamicImage::ImageRgba8(img.clone()).flipv();
                    dyn_image.to_rgba().into_raw()
                } ).collect();
                let data : Vec<_> = images_raw.iter().map(|v| v.as_slice()).collect();

                let kind = texture_kind_for(&texture_array_data.dimensions);

                let (texture, texture_view) = self.factory.create_texture_immutable_u8::<Srgba8>(kind, data.as_slice()).map_err(PuckError::CombinedGFXError)?;

                Ok((texture, texture_view))
            });

            match texture_load_result {
                Ok((t, tv)) => {
                    let pair = (t, tv);
                    self.texture = Some(pair);
                },
                Err(e) => println!("texture load error -> {:?}", e),
            }
        }

        self.texture.is_some() && self.pipelines.is_some()
    }
}

pub fn texture_kind_for(dimensions: &TextureArrayDimensions) -> gfx::texture::Kind {
    gfx::texture::Kind::D2Array(dimensions.width as u16, dimensions.height as u16, dimensions.layers as u16, gfx::texture::AaMode::Single)
}

