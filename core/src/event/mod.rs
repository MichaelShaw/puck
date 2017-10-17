


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Event<Id, Entity, EntityEvent, RenderEvent>  {
    Shutdown,
    SpawnEvent(Id, Entity),
    Delete(Id),
    DeleteRange(Id, Id),
    EntityEvent(Id, EntityEvent),
    RenderEvent(RenderEvent),
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

    pub fn clear(&mut self) {
        self.events.clear()
    }

    pub fn push(&mut self, a: A) {
        self.events.push(a);
    }

    pub fn push_opt(&mut self, a: Option<A>) {
        match a {
            Some(a) => self.push(a),
            None => (),
        }
    }
}

pub struct CombinedSink<A, B> {
    pub mine: Sink<A>,
    pub routed: Sink<B>,
}

impl<A, B> CombinedSink<A, B> {
    pub fn clear(&mut self) {
        self.mine.clear();
        self.routed.clear();
    }

    pub fn empty() -> CombinedSink<A, B> {
        CombinedSink  {
            mine: Sink::empty(),
            routed: Sink::empty(),
        }
    }
}