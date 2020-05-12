use crate::client::SongKickOpts;
use crate::options::{format_with_options, Options};
use crate::resources::event::Event;
use crate::resources::Resource;
use crate::result::SkResultSet;
use crate::util::encode;
use crate::SkResult;

use std::sync::Arc;

mod artists;
mod events;

pub use crate::endpoints::artists::ArtistEndpoint;
pub use crate::endpoints::events::EventEndpoint;

use reqwest;

#[doc(hidden)]
trait SkEndpointInternal {
    type Model: Resource;
    fn new() -> Self;
    fn get(
        &self,
        id: u64,
        sk: &SongKickOpts,
        ctx_path: &str,
    ) -> SkResult<SkResultSet<Self::Model>> {
        let url = format!(
            "{}/{}/{}.json?apikey={}",
            sk.base_path(),
            ctx_path,
            id,
            sk.api_key()
        );

        self.fetch(&url, sk, None)
    }

    fn calendar(
        &self,
        id: u64,
        sk: &SongKickOpts,
        ctx_path: &str,
        options: Option<Options>,
    ) -> SkResult<SkResultSet<Event>> {
        let url = format!(
            "{}/{}/{}/calendar.json?apikey={}",
            sk.base_path(),
            ctx_path,
            id,
            sk.api_key()
        );
        self.fetch(&url, sk, options)
    }
    fn search_by_name(
        &self,
        text: &str,
        sk: &SongKickOpts,
        ctx_path: &str,
        options: Option<Options>,
    ) -> SkResult<SkResultSet<Self::Model>> {
        let url = format!(
            "{}/search/{}.json?query={}&apikey={}",
            sk.base_path(),
            ctx_path,
            encode(text),
            sk.api_key()
        );
        self.fetch(&url, sk, options)
    }

    fn gigography(
        &self,
        id: u64,
        sk: &SongKickOpts,
        ctx_path: &str,
        options: Option<Options>,
    ) -> SkResult<SkResultSet<Event>> {
        let url = format!(
            "{}/{}/{}/gigography.json?apikey={}",
            sk.base_path(),
            ctx_path,
            id,
            sk.api_key()
        );

        self.fetch(&url, sk, options)
    }

    fn fetch<M>(
        &self,
        base_path: &str,
        sk: &SongKickOpts,
        options: Option<Options>,
    ) -> SkResult<SkResultSet<M>>
    where
        M: Resource,
    {
        let url = format_with_options(&base_path, options);
        let full_resp = reqwest::blocking::get(&url)?.text()?;

        let data = serde_json::from_str(&full_resp)?;

        SkResultSet::from_json(&data)
    }
}

pub trait SkEndpoint {
    type Model: Resource;
    fn new(sk: Arc<SongKickOpts>) -> Self;
    fn get(&self, id: u64) -> SkResult<SkResultSet<Self::Model>>;
}
