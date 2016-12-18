use hyper::*;
use endpoints::{ArtistEndpoint, SkEndpoint, EventEndpoint};
use std::sync::Arc;

pub struct SongKick {
    pub artist: ArtistEndpoint,
    pub event: EventEndpoint,
    #[allow(dead_code)]
    opts: Arc<SongKickOpts>
}

pub struct SongKickOpts {
    api_key: String,
    client: Arc<Client>,
    base_path: &'static str
}


impl SongKickOpts {
    pub fn new<T>(api_key: T, client: Arc<Client>, base_path: &'static str) -> SongKickOpts where T: Into<String> {
        SongKickOpts {
            api_key: api_key.into(),
            client: client,
            base_path: base_path
        }
    }
    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn base_path(&self) -> &str {
        self.base_path
    }
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

impl SongKick {
    pub fn new<T>(api_key: T) -> SongKick where T: Into<String> {
        let hyper = Arc::new(Client::new());

        let opts = Arc::new(SongKickOpts {
            api_key: api_key.into(),
            client: hyper,
            base_path: "http://api.songkick.com/api/3.0"
        });
        let artist = ArtistEndpoint::new(opts.clone());
        let event = EventEndpoint::new(opts.clone());
        SongKick {
            artist: artist,
            event: event,
            opts: opts
        }
    }
}

