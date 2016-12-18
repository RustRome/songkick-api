use result::{SkResultSet};
use SkResult;
use resources::Resource;
use resources::event::Event;
use std::sync::Arc;
use client::SongKickOpts;
use serde_json;
use std::io::Read;
use util::encode;
use options::{format_with_options, Options};

mod artists;
mod events;

pub use endpoints::artists::ArtistEndpoint as ArtistEndpoint;
pub use endpoints::events::EventEndpoint as EventEndpoint;


trait SkEndpointInternal {
    type Model: Resource;
    fn new() -> Self;
    fn get(&self, id: u64, sk: &SongKickOpts, ctx_path: &str) -> SkResult<SkResultSet<Self::Model>> {
        let url = format!("{}/{}/{}.json?apikey={}", sk.base_path(), ctx_path, id, sk.api_key());

        self.fetch(&url, sk, None)
    }

    fn calendar(&self, id: u64, sk: &SongKickOpts, ctx_path: &str, options: Option<Options>) -> SkResult<SkResultSet<Event>> {
        let url = format!("{}/{}/{}/calendar.json?apikey={}", sk.base_path(), ctx_path, id, sk.api_key());
        self.fetch(&url, sk, options)
    }
    fn search_by_name(&self, text: &str, sk: &SongKickOpts, ctx_path: &str, options: Option<Options>) -> SkResult<SkResultSet<Self::Model>> {
        let url = format!("{}/search/{}.json?query={}&apikey={}", sk.base_path(), ctx_path, encode(text), sk.api_key());
        self.fetch(&url, sk, options)
    }

    fn gigography(&self, id: u64, sk: &SongKickOpts, ctx_path: &str, options: Option<Options>) -> SkResult<SkResultSet<Event>> {
        let url = format!("{}/{}/{}/gigography.json?apikey={}", sk.base_path(), ctx_path, id, sk.api_key());

        self.fetch(&url, sk, options)
    }

    fn fetch<M>(&self, base_path: &str, sk: &SongKickOpts, options: Option<Options>) -> SkResult<SkResultSet<M>> where M: Resource {
        let url = format_with_options(&base_path, options);
        let mut res = try!(sk.client().get(&url).send());

        let mut full_resp = String::new();

        try!(res.read_to_string(&mut full_resp));

        let data = try!(serde_json::from_str(&full_resp));

        SkResultSet::from_json(&data)
    }
}


pub trait SkEndpoint {
    type Model: Resource;
    fn new(sk: Arc<SongKickOpts>) -> Self;
    fn get(&self, id: u64) -> SkResult<SkResultSet<Self::Model>>;
}


