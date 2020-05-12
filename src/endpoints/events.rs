use crate::resources::event::Event;
use crate::result::{SkResultSet};
use crate::SkResult;
use crate::client::SongKickOpts;
use std::sync::Arc;
use crate::endpoints::SkEndpoint;
use crate::endpoints::SkEndpointInternal;
use crate::options::Options;


#[doc(hidden)]
struct EventEndpointDelegate {}

impl SkEndpointInternal for EventEndpointDelegate {
    type Model = Event;
    fn new() -> EventEndpointDelegate {
        EventEndpointDelegate {}
    }
}

/// Public Struct for Artist Endpoint
pub struct EventEndpoint {
    /// SongKick Options
    sk: Arc<SongKickOpts>,
    /// Internal Delegate
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
    /// Get a Single [Event](https://www.songkick.com/developer/events-details) with ID
    fn get(&self, id: u64) -> SkResult<SkResultSet<Self::Model>> {
        self.delegate.get(id, self.sk.as_ref(), "events")
    }
}

impl EventEndpoint {

    /// Search for [Events](https://www.songkick.com/developer/event-search)
    pub fn search(&self, options: Options) -> SkResult<SkResultSet<Event>> {
        let url = format!("{}/events.json?apikey={}", self.sk.base_path(), self.sk.api_key());

        self.delegate.fetch::<Event>(&url, self.sk.as_ref(), Some(options))
    }
}







