use SkResult;
use resources::Resource;
use serde_json::Value;
use error::SkError;
use resources::identifier::Identifier;
use util::json::{get_str, get_u64};

#[derive(Debug, PartialEq)]
pub struct Artist {
    pub display_name: String,
    pub uri: String,
    pub id: u64,
    pub identfiers: Vec<Identifier>
}

impl Resource for Artist {
    fn from_json(source: &Value) -> SkResult<Artist> {
        match source.as_object() {
            Some(obj) => {
                let display_name = try!(get_str(obj, "displayName"));

                let uri = try!(get_str(obj, "uri"));

                let id = try!(get_u64(obj, "id"));

                let mut identifiers = Vec::new();

                if let Some(arr) = obj.get("identifier") {
                    for a in arr.as_array().unwrap() {
                        let identi = try!(Identifier::from_json(&a));
                        identifiers.push(identi);
                    }
                }

                let artist = Artist {
                    id: id,
                    uri: uri,
                    display_name: display_name,
                    identfiers: identifiers
                };
                Ok(artist)
            },
            None => Err(SkError::JsonError(format!("Expected source json to be an object {}", &source)))
        }
    }
    fn marker() -> &'static str {
        "artist"
    }
}


#[allow(unused_imports)]
#[allow(dead_code)]

mod tests {
    use std::fs::File;
    use std::io::Read;
    use resources::artist::Artist;
    use resources::{Resource};
    use serde_json::Value;
    use serde_json;

    fn load_artist(path: &str) -> Artist {
        let sample_str = {
            let mut file = File::open(path).unwrap();
            let mut ret = String::new();
            file.read_to_string(&mut ret).unwrap();
            ret
        };
        let data: Value = serde_json::from_str(&sample_str).unwrap();
        Artist::from_json(&data).unwrap()
    }


    #[test]
    fn test_artist_json() {
        let artist = load_artist("fixtures/artist/artist-253846.json");

        assert_eq!("Radiohead", artist.display_name);
        assert_eq!("http://www.songkick.com/artists/253846-radiohead?utm_source=24619&utm_medium=partner", artist.uri);
        assert_eq!(253846, artist.id);
        assert_eq!("a74b1b7f-71a5-4011-9441-d0b5e4122711", artist.identfiers[0].mbid);
    }
}