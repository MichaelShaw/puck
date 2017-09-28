pub mod runner;

use std::fmt::Debug;

use std::collections::BTreeMap as Map;

use Tick;
use std::hash::Hash;

use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Event<Id, Entity, EntityEvent, RenderEvent>  {
    SpawnEvent(Id, Entity),
    Delete(Id),
    DeleteRange(Id, Id),
    EntityEvent(Id, EntityEvent),
    RenderEvent(RenderEvent),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SimSettings {
    pub ticks_per_second: u64,
}

pub trait App {
    type Id : Clone + Hash + Debug + Eq + Ord + Serialize + DeserializeOwned;
    type Entity : Clone + Debug + Serialize + DeserializeOwned; // do we need Eq?
    type EntityEvent : Clone + Debug + Eq + Ord + Serialize + DeserializeOwned;
    type RenderEvent : Clone + Debug + Eq + Ord + Serialize + DeserializeOwned;

    fn handle_entity_event(event:&Self::EntityEvent, entity: &mut Self::Entity) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>;
    fn simulate(time:Tick, state:&Map<Self::Id, Self::Entity>, id: &Self::Id, entity: &Self::Entity) -> (Vec<Self::EntityEvent>, Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>);
}