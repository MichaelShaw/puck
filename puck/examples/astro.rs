extern crate puck;
extern crate puck_core;

use puck_core::Vec2;


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Id {
    Player,
    Rock(u64),
    Shot(u64),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EntityKind {
    Player,
    Rock,
    Shot
}

#[derive(Copy, Clone)]
pub struct Entity {
    pub kind: EntityKind,
    pub pos: Vec2,
    pub facing: f32,
    pub velocity: Vec2,
    pub rvel: f32,
    pub bbox_size: f32,
    // I am going to lazily overload "life" with a
    // double meaning:
    // for shots, it is the time left to live,
    // for players and rocks, it is the actual hit points.
    pub life: f32,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EntityEvent {
    UpdateFacing { velocity : Vec2, facing: f64, position: Vec2 },
}

pub fn main() {
    println!("astroblasto");
}