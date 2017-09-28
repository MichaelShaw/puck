extern crate puck;
extern crate puck_core;
extern crate cgmath;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use cgmath::Zero;

use puck_core::{Vec2, Vec3f, Tick, HashMap};
use puck_core::app::{TreeMap, App, Event, SimSettings};
use puck::app::{RenderedApp, RenderSettings};
use puck::{FileResources, RenderTick, Input, Dimensions};
use puck::audio::{SoundRender, Listener, SoundEvent};
use puck::render::gfx::OpenGLRenderer;

use std::hash::Hash;



#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Id {
    Player,
    Rock(u64),
    Shot(u64),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum EntityKind {
    Player,
    Rock,
    Shot
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub kind: EntityKind,
    pub pos: Vec2,
    pub facing: f32,
    pub velocity: Vec2,
    pub rvel: f32,
    pub bbox_size: f32,
    pub life: f32, // for shots, times alive, for players/rocks hp
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum EntityEvent {
    UpdateFacing { velocity : Vec2, facing: f64, position: Vec2 },
}

pub fn main() {
    println!("astroblasto");
    let file_resources = FileResources::default_relative();
    let sim_settings = SimSettings { tick_rate: 60 };
    let render_settings = RenderSettings { dimensions: (640, 480), vsync: false, title: "Astroblasto!".into() };

    let run_result = puck::app::runner::run::<AstroApp>(file_resources, sim_settings, render_settings, Vec::new());
}

struct AstroApp();

impl App for AstroApp {
    type Id = Id;
    type Entity = Entity; // do we need Eq?
    type EntityEvent = EntityEvent;
    type RenderEvent = SoundEvent;

    fn handle_entity_event(event:&Self::EntityEvent, entity: &mut Self::Entity) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>> {
        Vec::new()
    }

    fn simulate(time:Tick, entities:&TreeMap<Self::Id, Self::Entity>, id: &Self::Id, entity: &Self::Entity) -> (Vec<Self::EntityEvent>, Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>) {
        (Vec::new(), Vec::new())
    }
}

impl RenderedApp for AstroApp {
    type RenderState = Vec<SoundEvent>;

    fn handle_input(input:&Input, dimensions: &Dimensions, entities: &TreeMap<Self::Id, Self::Entity>) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>> {
        Vec::new()
    }

    fn handle_render_event(event: &Self::RenderEvent, render_state: &mut Self::RenderState) {
        render_state.push(event.clone());
    }

    fn render(time: RenderTick, entities:&TreeMap<Self::Id, Self::Entity>, render_state: &mut Self::RenderState, renderer: &mut OpenGLRenderer) -> SoundRender {

        let out = render_state.split_off(0);
        SoundRender {
            master_gain: 1.0,
            sounds: Vec::new(),
            persistent_sounds: HashMap::default(),
            listener: Listener {
                position: Vec3f::zero(),
                velocity: Vec3f::zero(),
                orientation_up: Vec3f::zero(),
                orientation_forward: Vec3f::zero(),
            }
        }
    }
}

