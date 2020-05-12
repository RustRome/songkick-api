use crate::SkResult;
use crate::resources::Resource;
use serde_json::Value;
use crate::resources::country::Country;
use crate::error::SkError;
use crate::util::json::{get_str, get_u64};

pub struct MetroArea {
    pub id: u64,
    pub display_name: String,
    pub uri: String,
    pub country: Country
}


impl Resource for MetroArea {
    fn from_json(source: &Value) -> SkResult<MetroArea> where Self: Sized {
        match source.as_object() {
            Some(obj) => {

                let id = get_u64(obj, "id")?;
                let display_name = get_str(obj, "displayName")?;
                let uri = get_str(obj, "uri")?;

                let country = obj.get("country").unwrap();

                let country = Country::from_json(&country)?;

                Ok(MetroArea {
                    id: id,
                    display_name: display_name,
                    uri: uri,
                    country: country
                })
            }
            None => Err(SkError::JsonError(format!("Expected source json to be an object {}", &source)))
        }
    }

    fn marker() -> &'static str {
        unimplemented!()
    }
}