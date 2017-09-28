pub mod runner;

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


pub trait RenderedApp : App {
    type RenderState;

    fn handle_input(input:&Input, dimensions: &Dimensions, state: &HashMap<Self::Id, Self::Entity>) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>;
    fn handle_render_event(event: &Self::RenderEvent, &mut Self::RenderState);
    fn render(state:&HashMap<Self::Id, Self::Entity>, &mut Self::RenderState);
}

