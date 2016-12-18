use result::{SkResultSet};
use SkResult;
use resources::artist::Artist;
use resources::event::Event;
use client::SongKickOpts;
use std::sync::Arc;
use endpoints::SkEndpointInternal;
use endpoints::SkEndpoint;
use options::Options;

struct ArtistEndpointDelegate {}

impl SkEndpointInternal for ArtistEndpointDelegate {
    type Model = Artist;
    fn new() -> ArtistEndpointDelegate {
        ArtistEndpointDelegate {}
    }
}

pub struct ArtistEndpoint {
    delegate: ArtistEndpointDelegate,
    sk: Arc<SongKickOpts>
}


impl SkEndpoint for ArtistEndpoint {
    type Model = Artist;


    fn new(sk: Arc<SongKickOpts>) -> ArtistEndpoint {
        let delegate = ArtistEndpointDelegate::new();
        ArtistEndpoint {
            delegate: delegate,
            sk: sk
        }
    }
    fn get(&self, id: u64) -> SkResult<SkResultSet<Self::Model>> {
        self.delegate.get(id, self.sk.as_ref(), "artists")
    }
}


impl ArtistEndpoint {
    pub fn search_by_name(&self, text: &str) -> SkResult<SkResultSet<Artist>> {
        self.delegate.search_by_name(text, self.sk.as_ref(), "artists", None)
    }

    pub fn calendar(&self, id: u64, options: Option<Options>) -> SkResult<SkResultSet<Event>> {
        self.delegate.calendar(id, self.sk.as_ref(), "artists", options)
    }

    pub fn gigography(&self, id: u64, options: Option<Options>) -> SkResult<SkResultSet<Event>> {
        self.delegate.gigography(id, self.sk.as_ref(), "artists", options)
    }
}