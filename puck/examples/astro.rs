extern crate puck;
#[macro_use]
extern crate puck_core;
extern crate cgmath;
extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use cgmath::{Zero, InnerSpace, vec3, vec2, Rad};
use std::f64::consts::PI;


use puck_core::{Vec2f, Vec3f, Vec3, Tick, HashMap, TreeMap, Color};
use puck_core::app::{App, Event, SimSettings};

use puck::app::{RenderedApp, RenderSettings};
use puck::{FileResources, RenderTick, Input, Dimensions, Camera};
use puck::audio::{SoundRender, Listener, SoundEvent};
use puck::render::gfx::OpenGLRenderer;
use puck::render::*;

use std::collections::Bound;
use std::collections::Bound::*;

use std::hash::Hash;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Id {
    Game,
    Player,
    Rock(u64),
    Shot(u64),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum ActorKind {
    Player,
    Rock,
    Shot
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Entity {
    Game { level: u64, score: u64 },
    Actor(Actor),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Vec2f,
    pub facing: f32,
    pub velocity: Vec2f,
    pub rvel: f32,
    pub bbox_size: f32,
    pub life: f32, // for shots, times alive, for players/rocks hp
    pub thrust: bool,
}

const PLAYER_LIFE: f32 = 1.0;
const SHOT_LIFE: f32 = 2.0;
const ROCK_LIFE: f32 = 1.0;

const PLAYER_BBOX: f32 = 12.0;
const ROCK_BBOX: f32 = 12.0;
const SHOT_BBOX: f32 = 6.0;

const MAX_ROCK_VEL: f32 = 50.0;

const SHOT_SPEED: f32 = 200.0;
const SHOT_RVEL: f32 = 0.1;
const SPRITE_SIZE: u32 = 32;

// Acceleration in pixels per second, more or less.
const PLAYER_THRUST: f32 = 100.0;
// Rotation in radians per second.
const PLAYER_TURN_RATE: f32 = 3.05;
// Seconds between shots
const PLAYER_SHOT_TIME: f32 = 0.5;

const MAX_PHYSICS_VEL: f32 = 250.0;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum EntityEvent {
    UpdateShipControls { rvel: f32, thrust: bool },
    UpdatePhysics { velocity : Vec2f, facing: f32, position: Vec2f },
    AdjustLife { life: f32 },
    IncreaseLevel,
    IncreaseScore,
}

fn create_player() -> Actor {
    Actor {
        kind: ActorKind::Player,
        pos: Vec2f::zero(),
        facing: 0.,
        velocity: Vec2f::zero(),
        rvel: 0.,
        bbox_size: PLAYER_BBOX,
        life: PLAYER_LIFE,
        thrust: false,
    }
}

fn create_rock() -> Actor {
    Actor {
        kind: ActorKind::Rock,
        pos: Vec2f::zero(),
        facing: 0.,
        velocity: Vec2f::zero(),
        rvel: 0.,
        bbox_size: ROCK_BBOX,
        life: ROCK_LIFE,
        thrust: false,
    }
}

fn create_shot() -> Actor {
    Actor {
        kind: ActorKind::Shot,
        pos: Vec2f::zero(),
        facing: 0.,
        velocity: Vec2f::zero(),
        rvel: SHOT_RVEL,
        bbox_size: SHOT_BBOX,
        life: SHOT_LIFE,
        thrust: false,
    }
}

fn random_vec(max_magnitude: f32) -> Vec2f {
    let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    let mag = rand::random::<f32>() * max_magnitude;
    vec_from_angle(angle) * (mag)
}

fn vec_from_angle(angle: f32) -> Vec2f {
    let vx = angle.sin();
    let vy = angle.cos();
    Vec2f::new(vx, vy)
}

fn create_rocks(num: u64, exclusion: Vec2f, min_radius: f32, max_radius: f32) -> Vec<(Id, Entity)> {
    assert!(max_radius > min_radius);
    let new_rock = |n| {
        let mut rock = create_rock();
        let r_angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
        let r_distance = rand::random::<f32>() * (max_radius - min_radius) + min_radius;
        rock.pos = exclusion + vec_from_angle(r_angle) * r_distance;
        rock.velocity = random_vec(MAX_ROCK_VEL);
        (Id::Rock(n), Entity::Actor(rock))
    };
    (0..num).map(new_rock).collect()
}

fn wrapped_position(pos:Vec2f, wrap_x:f32, wrap_y:f32) -> Vec2f {
    let mut wrapped_pos = pos;

    if pos.x > wrap_x {
        wrapped_pos.x -= wrap_x;
    } else if pos.x < 0. {
        wrapped_pos.x += wrap_x;
    };
    if pos.y > wrap_y {
        wrapped_pos.y -= wrap_y;
    } else if pos.y < 0. {
        wrapped_pos.y += wrap_y;
    }
    wrapped_pos
}

fn update_physics(actor:&Actor, time:f32, wrap_x:f32, wrap_y: f32) -> EntityEvent {
    // Clamp the velocity to the max efficiently

    let norm = actor.velocity.magnitude();
    let vel = if norm > MAX_PHYSICS_VEL {
        actor.velocity / norm * MAX_PHYSICS_VEL
    } else {
        actor.velocity
    };

    let accel = if actor.thrust {
        let v = vec_from_angle(-actor.facing + PI as f32);
        v * PLAYER_THRUST
    } else {
        vec2(0., 0.)
    };

    EntityEvent::UpdatePhysics {
        velocity : vel + accel * time,
        facing: actor.facing + actor.rvel * time,
        position: wrapped_position(actor.pos + actor.velocity * time, wrap_x, wrap_y),
    }
}

pub fn main() {
    println!("astroblasto");
    let file_resources = FileResources::default_relative();
    let sim_settings = SimSettings { tick_rate: 60 };
    let render_settings = RenderSettings { dimensions: (640, 480), vsync: false, title: "Astroblasto!".into() };

    let init_state = treemap![Id::Game => Entity::Game { level: 0, score: 0 }];

    let run_result = puck::app::runner::run::<AstroApp>(file_resources, sim_settings, render_settings, Vec::new(), init_state);
}

struct AstroApp();

pub type IdRange = (Bound<Id>, Bound<Id>);

pub const ALL_ROCKS : IdRange = (Included(Id::Rock(0)), Included(Id::Rock(100)));
pub const ALL_SHOTS : IdRange = (Included(Id::Shot(0)), Included(Id::Shot(100)));

pub fn no_events<A, B>() -> (Vec<A>, Vec<B>) {
    (Vec::new(), Vec::new())
}

//struct RenderState {
//    pub sound_events: Vec<SoundEvent>,
//
//}

impl App for AstroApp {
    type Id = Id;
    type Entity = Entity; // do we need Eq?
    type EntityEvent = EntityEvent;
    type RenderEvent = SoundEvent;

    fn handle_entity_event(event:&Self::EntityEvent, id: &Self::Id, entity: &mut Self::Entity) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>> {
        use Entity::*;
        use EntityEvent::*;

        match (event, entity) {
            (&IncreaseLevel, &mut Game { mut level, .. }) => { level += 1; vec![] },
            (&IncreaseScore, &mut Game { mut score, ..}) => { score += 1; vec![] },
            (&UpdateShipControls { rvel, thrust }, &mut Actor(ref mut actor)) => {
                actor.rvel = rvel;
                actor.thrust = thrust;
                vec![]
            }
            (&UpdatePhysics { velocity, facing, position }, &mut Actor(ref mut actor)) => {
                actor.velocity = velocity;
                actor.facing = facing;
                actor.pos = position;
                vec![]
            }
            (&AdjustLife { life }, &mut Actor(ref mut actor)) => {
                actor.life = life;
                vec![]
            },
            (ev, ent) => {
                println!("uhhh, dont recognize {:?} with {:?}", ev, ent);
                vec![]
            },
        }
    }

    fn simulate(time:Tick, entities:&TreeMap<Self::Id, Self::Entity>, id: &Self::Id, entity: &Self::Entity) -> (Vec<Self::EntityEvent>, Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>) {
        use puck_core::app::Event::*;
        use Entity::*;
        use EntityEvent::*;
        match entity {
            &Game { score, level } => {
                if let Some(&Entity::Actor(player)) = entities.get(&Id::Player) {
                    let rock_count = entities.range(ALL_ROCKS).count();
                    if rock_count == 0 {
                        let spawn_rocks = create_rocks(level + 6, player.pos, 100.0, 250.0).into_iter().map(|(id, entity)| {
                            Event::SpawnEvent(id, entity)
                        }).collect();
                        (vec![IncreaseLevel], spawn_rocks)
                    } else {
                        no_events()
                    }
                } else {
                    // no player
                    (vec![], vec![SpawnEvent(Id::Player, Entity::Actor(create_player()))])
                }
            },
            &Actor(ref actor) => {
//                println!("simulate -> {:?} for {:?}", actor, time.tick_duration);

                (vec![update_physics(actor, time.tick_duration as f32, 640.0, 480.0)], vec![] )
            },
        }
    }
}

impl RenderedApp for AstroApp {
    type RenderState = Vec<SoundEvent>;

    fn handle_input(input:&Input, dimensions: &Dimensions, entities: &TreeMap<Self::Id, Self::Entity>) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>> {

        if let Some(&Entity::Actor(actor)) = entities.get(&Id::Player) {
           let rvel = if input.keys.down.contains(&puck::input::VirtualKeyCode::A) {
               -PLAYER_TURN_RATE
           } else if input.keys.down.contains(&puck::input::VirtualKeyCode::D) {
                PLAYER_TURN_RATE
           } else {
               0.0
           };
           let thrust = input.keys.down.contains(&puck::input::VirtualKeyCode::W);
           vec![Event::EntityEvent(Id::Player, EntityEvent::UpdateShipControls { rvel, thrust})]
        } else {
           vec![]
        }
    }

    fn handle_render_event(event: &Self::RenderEvent, render_state: &mut Self::RenderState) {
        render_state.push(event.clone());
    }

    fn render(time: RenderTick, dimensions: &Dimensions, entities:&TreeMap<Self::Id, Self::Entity>, render_state: &mut Self::RenderState, renderer: &mut OpenGLRenderer) -> SoundRender {
        use Entity::*;
        use ActorKind::*;

        renderer.clear_depth_and_color(Color::BLACK);

        let tesselator = GeometryTesselator::new(Vec3::new(1.0, 1.0, 1.0));
        let mut verticies = Vec::new();

        let atlas = TextureAtlas {
            texture_size: 512,
            tile_size: 32,
        };
        let rock = atlas.at(0, 0);
        let shot = atlas.at(1, 0);
        let player = atlas.at(2, 0);

        for (id, e) in entities {
            match e {
                &Game { level, score } => {

                },
                &Actor(actor) => {
                    let tex = match actor.kind {
                        Shot => shot,
                        Player => player,
                        Rock => rock,
                    };

                    tesselator.draw_floor_centre_anchored_rotated_at(&mut verticies, &tex, vec3(actor.pos.x as f64, 0.0, actor.pos.y as f64), actor.facing as f64, 0.0);
                },
            }
        }

        let (w, h) = dimensions.points;
        let camera = Camera {
            at: vec3(w as f64 / 2.0, 0., h as f64 / 2.0),
            pitch: Rad(PI / 4.0_f64),
            viewport: *dimensions,
            points_per_unit: 1.0,
            near_far: (-1000., 1000.),
        };

        renderer.draw_vertices(&verticies, Uniforms {
            transform : camera.view_projection(),
            color: Color::WHITE,
        }, Blend::None).expect("the ability to draw");
        renderer.finish_frame().expect("finished frame");

        let out = render_state.split_off(0);
        SoundRender::non_positional_effects(Vec::new())
    }
}

