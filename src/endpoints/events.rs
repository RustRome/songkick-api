use resources::event::Event;
use result::{SkResultSet};
use SkResult;
use client::SongKickOpts;
use std::sync::Arc;
use endpoints::SkEndpoint;
use endpoints::SkEndpointInternal;
use options::Options;

struct EventEndpointDelegate {}

impl SkEndpointInternal for EventEndpointDelegate {
    type Model = Event;
    fn new() -> EventEndpointDelegate {
        EventEndpointDelegate {}
    }
}

pub struct EventEndpoint {
    sk: Arc<SongKickOpts>,
    delegate: EventEndpointDelegate
}


impl SkEndpoint for EventEndpoint {
    type Model = Event;

    fn new(sk: Arc<SongKickOpts>) -> EventEndpoint {
        let delegate = EventEndpointDelegate::new();
        EventEndpoint {
            sk: sk,
            delegate: delegate
        }
    }
    fn get(&self, id: u64) -> SkResult<SkResultSet<Self::Model>> {
        self.delegate.get(id, self.sk.as_ref(), "events")
    }
}

impl EventEndpoint {
    pub fn search(&self, options: Options) -> SkResult<SkResultSet<Event>> {
        let url = format!("{}/events.json?apikey={}", self.sk.base_path(), self.sk.api_key());

        self.delegate.fetch::<Event>(&url, self.sk.as_ref(), Some(options))
    }
}







