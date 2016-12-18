use SkResult;
use serde_json::Value;
use std::marker::Sized;

pub mod artist;
pub mod identifier;
pub mod event;
pub mod venue;
pub mod metro_area;
pub mod country;

pub use resources::artist::Artist as Artist;
pub use resources::event::Event as Event;

pub trait Resource {
    fn from_json(source: &Value) -> SkResult<Self> where Self: Sized;

    fn marker() -> &'static str;
}


