use gfx;
use std::collections::BTreeMap;

use render::gfx::{Renderer, construct_opengl_renderer};
use FileResources;

use PuckResult;
use puck_core::app::SimSettings;
use super::{RenderedApp, RenderSettings};

pub struct ReneredAppRunner<RA, R, C, F, D> where RA : RenderedApp,
                                                  R : gfx::Resources,
                                                  C : gfx::CommandBuffer<R>,
                                                  F : gfx::Factory<R>,
                                                  D : gfx::Device {
    entities: BTreeMap<RA::Id, RA::Entity>,
    renderer: Renderer<R, C, F, D>,
    render_state: RA::RenderState,
}


pub fn run<RA>(app: RA, file_resources:FileResources, sim_settings: SimSettings, render_settings:RenderSettings, render_state: RA::RenderState) -> PuckResult<()> where RA : RenderedApp {
    let renderer = construct_opengl_renderer(file_resources, render_settings.dimensions, render_settings.vsync, &render_settings.title)?;


    Ok(())
}