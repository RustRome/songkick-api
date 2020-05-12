use crate::client::SongKickOpts;
use crate::endpoints::SkEndpoint;
use crate::endpoints::SkEndpointInternal;
use crate::options::Options;
use crate::resources::artist::Artist;
use crate::resources::event::Event;
use crate::result::SkResultSet;
use crate::SkResult;
use std::sync::Arc;

#[doc(hidden)]
struct ArtistEndpointDelegate {}

impl SkEndpointInternal for ArtistEndpointDelegate {
    type Model = Artist;
    fn new() -> ArtistEndpointDelegate {
        ArtistEndpointDelegate {}
    }
}

/// Public Struct for Artist Endpoint
pub struct ArtistEndpoint {
    /// Internal Delegate
    delegate: ArtistEndpointDelegate,
    /// SongKick Options
    sk: Arc<SongKickOpts>,
}

impl SkEndpoint for ArtistEndpoint {
    type Model = Artist;

    fn new(sk: Arc<SongKickOpts>) -> ArtistEndpoint {
        let delegate = ArtistEndpointDelegate::new();
        ArtistEndpoint {
            delegate: delegate,
            sk: sk,
        }
    }

    /// Get Single Artist with ID
    fn get(&self, id: u64) -> SkResult<SkResultSet<Self::Model>> {
        self.delegate.get(id, self.sk.as_ref(), "artists")
    }
}

impl ArtistEndpoint {
    /// Search [Artists](https://www.songkick.com/developer/artist-search) by name
    pub fn search_by_name<T>(&self, text: T) -> SkResult<SkResultSet<Artist>>
    where
        T: Into<String>,
    {
        self.delegate
            .search_by_name(&text.into(), self.sk.as_ref(), "artists", None)
    }

    /// Retrieve [Calendar](https://www.songkick.com/developer/upcoming-events-for-artist) for an Artist with ID
    pub fn calendar(&self, id: u64, options: Option<Options>) -> SkResult<SkResultSet<Event>> {
        self.delegate
            .calendar(id, self.sk.as_ref(), "artists", options)
    }

    /// Retrieve [Gigography](https://www.songkick.com/developer/past-events-for-artist) for an Artist with ID
    pub fn gigography(&self, id: u64, options: Option<Options>) -> SkResult<SkResultSet<Event>> {
        self.delegate
            .gigography(id, self.sk.as_ref(), "artists", options)
    }
}
