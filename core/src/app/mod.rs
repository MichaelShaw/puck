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

pub struct Sink<A> {
    pub events: Vec<A>,
}

impl<A> Sink<A> {
    pub fn empty() -> Sink<A> {
        Sink {
            events: Vec::new()
        }
    }
    pub fn push(&mut self, a: A) {
        self.events.push(a);
    }
}

pub struct CombinedSink<A, B> {
    pub self_events: Vec<A>,
    pub events: Vec<B>,
}

impl<A, B> CombinedSink<A, B> {
    pub fn empty() -> CombinedSink<A, B> {
        CombinedSink  {
            self_events: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn push_self(&mut self, a: A) {
        self.self_events.push(a);
    }

    pub fn push_event(&mut self, b: B) {
        self.events.push(b);
    }
}

pub trait App {
    type Id : Clone + Hash + Debug + Eq + Ord + Serialize + DeserializeOwned;
    type Entity : Clone + Debug + Serialize + DeserializeOwned; // do we need Eq?
    type EntityEvent : Clone + Debug + Serialize + DeserializeOwned;
    type RenderEvent : Clone + Debug + Serialize + DeserializeOwned;

    fn handle_entity_event(event:&Self::EntityEvent, id: &Self::Id, entity: &mut Self::Entity, sink: &mut Sink<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>);
    fn simulate(time:Tick, entities:&TreeMap<Self::Id, Self::Entity>, id: &Self::Id, entity: &Self::Entity, sink: &mut CombinedSink<Self::EntityEvent, Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>);
}