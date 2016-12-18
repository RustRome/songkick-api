use SkResult;
use resources::Resource;
use serde_json::Value;
use error::SkError;
use util::json::{get_str};

pub struct Country {
    pub display_name: String
}

impl Resource for Country {
    fn from_json(source: &Value) -> SkResult<Self> where Self: Sized {
        match source.as_object() {
            Some(obj) => {
                let display_name = try!(get_str(obj, "displayName"));
                Ok(Country {
                    display_name: display_name
                })
            }
            None => Err(SkError::JsonError(format!("Expected source json to be an object {}", &source)))
        }
    }

    fn marker() -> &'static str {
        unimplemented!()
    }
}