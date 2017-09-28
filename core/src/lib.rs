extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

extern crate tokio_core;
extern crate tokio_io;
extern crate futures;
extern crate bytes;

extern crate cgmath;

extern crate fnv;

pub mod event;
pub mod network;
pub mod color;
pub mod math;
pub mod app;

pub use color::*;

pub type Vec2 = cgmath::Vector2<f64>;
pub type Vec2f = cgmath::Vector2<f32>;
pub type Vec2i = cgmath::Vector2<i32>;
pub type Vec2Size = cgmath::Vector2<usize>;

pub type Vec3 = cgmath::Vector3<f64>;
pub type Vec3f = cgmath::Vector3<f32>;
pub type Vec3i = cgmath::Vector3<i32>;

pub type Vec4 = cgmath::Vector4<f64>;

pub type Mat3 = cgmath::Matrix3<f64>;
pub type Mat4 = cgmath::Matrix4<f64>;

pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;
pub type HashSet<K> = fnv::FnvHashSet<K>;

pub fn clamp<T : PartialOrd>(n:T, min:T, max:T) -> T {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

#[derive(Copy, Clone)]
pub struct Tick {
    pub n: u64,
    pub tick_duration: f64,
    pub tick_rate: u64, // per second
}