use crate::endpoints::{ArtistEndpoint, EventEndpoint, SkEndpoint};
use std::sync::Arc;

/// Represent the SongKick client used to fetch the data from SongKick API
pub struct SongKick {
    /// Artist EndPoint
    pub artist: ArtistEndpoint,
    /// Event EndPoint
    pub event: EventEndpoint,
    #[allow(dead_code)]
    opts: Arc<SongKickOpts>,
}
/// Struct that holds SonKick Options
pub struct SongKickOpts {
    /// API KEY
    api_key: String,
    /// API base path
    base_path: &'static str,
}

impl SongKickOpts {
    pub fn new<T>(api_key: T, base_path: &'static str) -> SongKickOpts
    where
        T: Into<String>,
    {
        SongKickOpts {
            api_key: api_key.into(),
            base_path: base_path,
        }
    }

    /// Return base_path
    pub fn base_path(&self) -> &str {
        self.base_path
    }
    /// Return API Key

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

impl SongKick {
    pub fn new<T>(api_key: T) -> SongKick
    where
        T: Into<String>,
    {
        let opts = Arc::new(SongKickOpts {
            api_key: api_key.into(),
            base_path: "http://api.songkick.com/api/3.0",
        });
        let artist = ArtistEndpoint::new(opts.clone());
        let event = EventEndpoint::new(opts.clone());
        SongKick {
            artist: artist,
            event: event,
            opts: opts,
        }
    }
}
