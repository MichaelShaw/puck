use puck_core::{HashMap, Tick};
use puck_core::app::{Event, App};
use input::Input;

use std::hash::Hash;
use std::fmt::Debug;

use std::collections::BTreeMap;

use dimensions::Dimensions;

use render::gfx::{Renderer, construct_opengl_renderer};
use gfx;
use FileResources;
use PuckResult;

// - abstract trait of EventSink?
// - how do we manage identifiers? ... across kinds?
// - how do we determine viewability of render events?
// - initial state?
// - notions of client identity?
// - move entity state to tree map ...

// how does a server request/force a change to something it doesn't own? ... how does this affect ordering?
// how does client and server negotiate over player location? In PUBG, how do we put the player in the plane (with location being client side)

// "route", who's the owner of this entity
// "visibility" server notion foreach client

#[derive(Clone)]
pub struct RenderSettings {
    pub dimensions: (u32, u32),
    pub vsync: bool,
    pub title: String,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SimSettings {
    pub ticks_per_second: u64,
}

pub trait RenderedApp : App {
    type RenderState;

    fn handle_input(input:&Input, dimensions: &Dimensions, state: &HashMap<Self::Id, Self::Entity>) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>;
    fn handle_render_event(event: &Self::RenderEvent, &mut Self::RenderState);
    fn render(state:&HashMap<Self::Id, Self::Entity>, &mut Self::RenderState);
}

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