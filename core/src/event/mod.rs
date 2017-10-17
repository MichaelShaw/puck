


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
    pub mine: Vec<A>,
    pub routed: Vec<B>,
}

impl<A, B> CombinedSink<A, B> {
    pub fn empty() -> CombinedSink<A, B> {
        CombinedSink  {
            mine: Vec::new(),
            routed: Vec::new(),
        }
    }
}