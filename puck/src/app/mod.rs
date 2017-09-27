use puck_core::HashMap;
use input::Input;

use std::hash::Hash;
use std::fmt::Debug;

use dimensions::Dimensions;

// - abstract trait of EventSink?
// - authority function? view function? (combine the two?)
//   Id -> Change/View/None (for a given ... player)
// - how are render events generated?
// - routed event, vs event?
// - spawn event? (it's global mutation) .. an event to an empty id
// - how do we manage identifiers? ... across kinds?

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event<Id, Entity, EntityEvent, RenderEvent> {
    SpawnEvent(Id, Entity),
    EntityEvent(Id, EntityEvent),
    RenderEvent(RenderEvent),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Authority {
    Change,
    View,
    None,
}

pub type Time = u64;

pub trait App {
    type Id : Clone + Hash + Debug + Eq + Ord;
    type Entity : Clone + Debug; // do we need Eq?
    type Event : Clone + Debug + Eq + Ord;

    fn handle_event(event:&Self::Event, entity: &mut Self::Entity) -> Vec<(Self::Id, Self::Event)>;
    fn simulate(time:Time, state:&HashMap<Self::Id, Self::Entity>, id: &Self::Id, entity: &Self::Entity) -> (Vec<Self::Event>, Vec<(Self::Id, Self::Event)>);
}

pub trait RenderedApp : App {
    type RenderEvent : Clone + Debug + Eq + Ord;
    type RenderState;

    fn handle_input(input:&Input, dimensions: &Dimensions, state: &HashMap<Self::Id, Self::Entity>) -> Vec<Self::Event>;
//    fn simulate(time:Time, state:&HashMap<Self::Id, Self::Entity>, entity: &Self::Entity) -> Vec<Self::Event>;

    fn handle_render_event(event: Self::Event, &mut Self::RenderState);
    fn render(state:&HashMap<Self::Id, Self::Entity>, &mut Self::RenderState);

}

