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
use render::{down_size_m4, TextureArrayDimensions};
use FileResources;

use image::DynamicImage;

use puck_core::HashMap;

use render::{Blend, Uniforms};

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
    pub fn begin_frame(&mut self, reload_texture: bool, reload_program: bool) -> (Dimensions, Input) {
        self.load_resources(reload_texture, reload_program);

        let mut events : Vec<glutin::Event> = Vec::new();

        self.events_loop.poll_events(|ev| events.push(ev));

        let mut close_requested = false;
        let mut resize = false;

        for ev in &events {
            if let &glutin::Event::WindowEvent { ref event, .. } = ev {
                match event {
                    &glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => {
                        close_requested = true
                    },
                    &glutin::WindowEvent::Closed => {
                        close_requested = true
                    },
                    &glutin::WindowEvent::Resized(width, height) => {
                        self.window.resize(width, height);
                        resize = true;
                    },
                    _ => (),
                }
            }
        }

        if resize {
            //            println!("resize, PRE -> {:?}", get_dimensions(&self.window));
            gfx_window_glutin::update_views(&self.window, &mut self.screen_colour_target, &mut self.screen_depth_target);
            //            println!("POST -> {:?}", get_dimensions(&self.window));
        }

        self.input = input::produce(&self.input, &events);
        self.input.close = close_requested;

        let dimensions = get_dimensions(&self.window);
        self.dimensions = dimensions;

        // clear
        if !close_requested {
            //            self.screen_colour_target = 4;
//            let decoded = decode_color(clear_color);
//            self.encoder.clear(&self.screen_colour_target, decoded);
//            self.encoder.clear_depth(&self.screen_depth_target, 1.0);
            //            let ad = self.screen_colour_target.get_dimensions();
            //            println!("internal dimensions -> {:?}", ad);
        }

        (dimensions, self.input.clone())
    }

    pub fn clear_depth_and_color(&mut self, color: Color) {
        self.clear_depth();
        self.clear_color(color);
    }

    pub fn clear_depth(&mut self) {
        self.encoder.clear_depth(&self.screen_depth_target, 1.0);
    }

    pub fn clear_color(&mut self, color:Color) {
        let decoded = decode_color(color);
        self.encoder.clear(&self.screen_colour_target, decoded);
    }

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

    pub fn upload(&mut self, vertices: &[Vertex]) -> GeometryBuffer<gfx_device_gl::Resources> {
        let (buffer, slice) = self.factory.create_vertex_buffer_with_slice(vertices, ());
        GeometryBuffer {
            buffer,
            slice,
        }
    }

    fn draw_raw(&mut self, geometry: &GeometryBuffer<gfx_device_gl::Resources>, uniforms: Uniforms, blend:Blend) -> PuckResult<()> {
//        let tv = match texture_array {
//            TextureArraySource::UI => &self.ui.texture_view,
//            TextureArraySource::Primary => self.texture.as_ref().map(|&(_, ref v)| v).ok_or(JamError::NoTexture())?,
//        };

        let tv = self.texture.as_ref().map(|&(_, ref v)| v).ok_or(PuckError::NoTexture())?;

        //        let tv = self.texture.as_ref().map(|&(_, ref v)| v).ok_or(JamError::NoTexture())?;

        match blend {
            Blend::None => {
                let opaque_pipe = self.pipelines.as_mut().ok_or(PuckError::NoPipeline()).map(|p| &mut p.opaque )?;
                let opaque_data = pipe_opaque::Data {
                    vbuf: geometry.buffer.clone(),
                    texture: (tv.clone(), self.sampler.clone()),
                    locals: self.factory.create_constant_buffer(1),
                    out_color: self.screen_colour_target.clone(),
                    out_depth: self.screen_depth_target.clone(),
                };
                let locals = Locals {
                    u_transform: uniforms.transform,
                    u_color: uniforms.color.float_raw(),
                    u_alpha_minimum: 0.01,
                };
                self.encoder.update_constant_buffer(&opaque_data.locals, &locals);
                self.encoder.draw(&geometry.slice, &opaque_pipe.pipeline, &opaque_data);
            },
            Blend::Add => {
                //                println!("no add pipeline atm")
            },
            Blend::Alpha => {
                let blend_pipe = self.pipelines.as_mut().ok_or(PuckError::NoPipeline()).map(|p| &mut p.blend )?;
                let blend_data = pipe_blend::Data {
                    vbuf: geometry.buffer.clone(),
                    texture: (tv.clone(), self.sampler.clone()),
                    locals: self.factory.create_constant_buffer(1),
                    out_color: self.screen_colour_target.clone(),
                    out_depth: self.screen_depth_target.clone(),
                };
                let locals = Locals {
                    u_transform: uniforms.transform,
                    u_color: uniforms.color.float_raw(),
                    u_alpha_minimum: 0.01,
                };
                self.encoder.update_constant_buffer(&blend_data.locals, &locals);
                self.encoder.draw(&geometry.slice, &blend_pipe.pipeline, &blend_data);
            },
        }

        Ok(())
    }

    pub fn draw(&mut self, geometry: &GeometryBuffer<gfx_device_gl::Resources>, uniforms: Uniforms, blend:Blend) -> PuckResult<()> {
        self.draw_raw(geometry, uniforms, blend)
    }

    pub fn draw_vertices(&mut self, vertices: &[Vertex], uniforms: Uniforms, blend:Blend) -> PuckResult<GeometryBuffer<gfx_device_gl::Resources>> {
        let geometry = self.upload(vertices);
        let res = self.draw(&geometry, uniforms, blend);
        res.map(|()| geometry)
    }

    pub fn finish_frame(&mut self) -> PuckResult<()> {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().map_err(PuckError::ContextError)?;
        self.device.cleanup();
        Ok(())
    }
}

fn decode_color(c: Color) -> [f32; 4] {
    let f = |xu: u8| {
        let x = (xu as f32)  / 255.0;
        if x > 0.04045 {
            ((x + 0.055) / 1.055).powf(2.4)
        } else {
            x / 12.92
        }
    };
    [f(c.r), f(c.g), f(c.b), 0.0]
}

pub fn texture_kind_for(dimensions: &TextureArrayDimensions) -> gfx::texture::Kind {
    gfx::texture::Kind::D2Array(dimensions.width as u16, dimensions.height as u16, dimensions.layers as u16, gfx::texture::AaMode::Single)
}

