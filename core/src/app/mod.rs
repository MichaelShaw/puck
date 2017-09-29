pub mod runner;

use std::fmt::Debug;

use std::collections::BTreeMap;

use Tick;
use std::hash::Hash;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub type TreeMap<K, V> = BTreeMap<K, V>;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Event<Id, Entity, EntityEvent, RenderEvent>  {
    Shutdown,
    SpawnEvent(Id, Entity),
    Delete(Id),
    DeleteRange(Id, Id),
    EntityEvent(Id, EntityEvent),
    RenderEvent(RenderEvent),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SimSettings {
    pub tick_rate: u64,
}

pub trait App {
    type Id : Clone + Hash + Debug + Eq + Ord + Serialize + DeserializeOwned;
    type Entity : Clone + Debug + Serialize + DeserializeOwned; // do we need Eq?
    type EntityEvent : Clone + Debug + Serialize + DeserializeOwned;
    type RenderEvent : Clone + Debug + Serialize + DeserializeOwned;

    fn handle_entity_event(event:&Self::EntityEvent, id: &Self::Id, entity: &mut Self::Entity) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>;
    fn simulate(time:Tick, entities:&TreeMap<Self::Id, Self::Entity>, id: &Self::Id, entity: &Self::Entity) -> (Vec<Self::EntityEvent>, Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>);
}