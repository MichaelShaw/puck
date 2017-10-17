pub mod runner;

use std::fmt::Debug;

use std::collections::BTreeMap;

use Tick;
use std::hash::Hash;

use serde::Serialize;
use serde::de::DeserializeOwned;

use event::*;

pub type TreeMap<K, V> = BTreeMap<K, V>;


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SimSettings {
    pub tick_rate: u64,
}


pub trait App {
    type Id : Clone + Hash + Debug + Eq + Ord + Serialize + DeserializeOwned;
    type Entity : Clone + Debug + Serialize + DeserializeOwned; // do we need Eq?
    type EntityEvent : Clone + Debug + Serialize + DeserializeOwned;
    type RenderEvent : Clone + Debug + Serialize + DeserializeOwned;

    fn handle_entity_event(event:&Self::EntityEvent, id: &Self::Id, entity: &mut Self::Entity, sink: &mut Sink<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>);
    fn simulate(time:Tick, entities:&TreeMap<Self::Id, Self::Entity>, id: &Self::Id, entity: &Self::Entity, sink: &mut CombinedSink<Self::EntityEvent, Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>);
}