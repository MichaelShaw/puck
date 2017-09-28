use puck_core::HashMap;
use input::Input;

use std::hash::Hash;
use std::fmt::Debug;

use dimensions::Dimensions;

// - abstract trait of EventSink?
// - authority function? view function? (combine the two?)
//   Id -> Change/View/None (for a given ... player)
// - how do we manage identifiers? ... across kinds?
// - how do we determine viewability of render events?
// - initial state?
// - notions of client identity?
// - move state to tree map?

// how does a server request/force a change to something it doesn't own? ... how does this affect ordering?

// how does client and server negotiate over player location? In PUBG, how do we put the player in the plane (with location being client side)

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event<Id, Entity, EntityEvent, RenderEvent>  {
    SpawnEvent(Id, Entity),
    Delete(Id),
    DeleteRange(Id, Id),
    EntityEvent(Id, EntityEvent),
    RenderEvent(RenderEvent),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Authority {
    Change,
    View,
    None,
} // how does the server interpret being order for being able to change a remote actor? ... maybe it's more of a routing function ...

pub type Time = u64;

pub struct RenderConfig {
    pub dimensions: (u32, u32),
    pub title: String,
}

pub trait App {
    type Id : Clone + Hash + Debug + Eq + Ord;
    type Entity : Clone + Debug; // do we need Eq?
    type EntityEvent : Clone + Debug + Eq + Ord;
    type RenderEvent : Clone + Debug + Eq + Ord;



    fn handle_entity_event(event:&Self::EntityEvent, entity: &mut Self::Entity) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>;
    fn simulate(time:Time, state:&HashMap<Self::Id, Self::Entity>, id: &Self::Id, entity: &Self::Entity) -> (Vec<Self::EntityEvent>, Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>);
}

pub trait RenderedApp : App {
    type RenderState;

    fn render_config() -> RenderConfig;
    fn handle_input(input:&Input, dimensions: &Dimensions, state: &HashMap<Self::Id, Self::Entity>) -> Vec<Event<Self::Id, Self::Entity, Self::EntityEvent, Self::RenderEvent>>;
//    fn simulate(time:Time, state:&HashMap<Self::Id, Self::Entity>, entity: &Self::Entity) -> Vec<Self::Event>;

    fn handle_render_event(event: &Self::RenderEvent, &mut Self::RenderState);
    fn render(state:&HashMap<Self::Id, Self::Entity>, &mut Self::RenderState);
}

