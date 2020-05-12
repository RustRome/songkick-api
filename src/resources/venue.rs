use crate::SkResult;
use crate::resources::Resource;
use crate::resources::metro_area::MetroArea;
use serde_json::Value;
use crate::error::SkError;

pub struct Venue {
    pub id: Option<u64>,
    pub display_name: Option<String>,
    pub uri: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub metro_area: Option<MetroArea>
}


impl Resource for Venue {
    fn from_json(source: &Value) -> SkResult<Self> where Self: Sized {
        match source.as_object() {
            Some(obj) => {

                let id = obj.get("id")
                    .and_then(|val| val.as_u64());

                let lat = obj.get("lat")
                    .and_then(|val| val.as_f64());

                let lng = obj.get("lng")
                    .and_then(|val| val.as_f64());


                let uri = obj.get("uri")
                    .and_then(|val| val.as_str())
                    .and_then(|str| Some(String::from(str)));

                let display_name = obj.get("displayName")
                    .and_then(|val| val.as_str())
                    .and_then(|str| Some(String::from(str)));



                let metro_area = match obj.get("metroArea") {
                    Some(val) => {
                        let m = MetroArea::from_json(&val)?;
                        Some(m)
                    }
                    None => None
                };


                Ok(Venue {
                    id: id,
                    display_name: display_name,
                    uri: uri,
                    lat: lat,
                    lng: lng,
                    metro_area: metro_area,
                })
            }
            None => Err(SkError::JsonError(format!("Expected source json to be an object {}", &source)))
        }
    }

    fn marker() -> &'static str {
        unimplemented!()
    }
}