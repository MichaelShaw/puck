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


pub type Vec3 = cgmath::Vector3<f64>;
pub type Vec3f = cgmath::Vector3<f32>;


pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;
pub type HashSet<K> = fnv::FnvHashSet<K>;