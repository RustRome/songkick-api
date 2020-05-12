use crate::error::SkError;
use crate::resources::Resource;
use crate::util::json::get_str;
use crate::SkResult;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub href: String,
    pub events_href: Option<String>,
    pub setlists_href: Option<String>,
    pub mbid: String,
}

impl Resource for Identifier {
    fn from_json(source: &Value) -> SkResult<Identifier> {
        match source.as_object() {
            Some(obj) => {
                let href = get_str(obj, "href")?;

                let mut events_href = None;
                if let Some(ref evt) = obj.get("eventsHref") {
                    events_href = Some(String::from(evt.as_str().unwrap()));
                }
                let mut setlists_href = None;
                if let Some(ref set) = obj.get("setlistsHref") {
                    setlists_href = Some(String::from(set.as_str().unwrap()));
                }

                let mbid = get_str(obj, "mbid")?;

                Ok(Identifier {
                    mbid: mbid,
                    events_href: events_href,
                    setlists_href: setlists_href,
                    href: href,
                })
            }
            None => Err(SkError::JsonError(format!(
                "Expected source json to be an object {}",
                &source
            ))),
        }
    }
    fn marker() -> &'static str {
        "identifier"
    }
}
