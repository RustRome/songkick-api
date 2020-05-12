use crate::error::SkError;
use crate::resources::identifier::Identifier;
use crate::resources::Resource;
use crate::util::json::{get_str, get_u64};
use crate::SkResult;
use serde_json::Value;

/// Represent a SongKick Artist Resource
#[derive(Debug, PartialEq)]
pub struct Artist {
    /// Display name
    pub display_name: String,
    pub uri: String,
    /// ID
    pub id: u64,
    pub identifiers: Vec<Identifier>,
}

impl Resource for Artist {
    fn from_json(source: &Value) -> SkResult<Artist> {
        match source.as_object() {
            Some(obj) => {
                let display_name = get_str(obj, "displayName")?;

                let uri = get_str(obj, "uri")?;

                let id = get_u64(obj, "id")?;

                let mut identifiers = Vec::new();

                if let Some(arr) = obj.get("identifier") {
                    for a in arr.as_array().unwrap() {
                        let identi = Identifier::from_json(&a)?;
                        identifiers.push(identi);
                    }
                }

                let artist = Artist {
                    id: id,
                    uri: uri,
                    display_name: display_name,
                    identifiers: identifiers,
                };
                Ok(artist)
            }
            None => Err(SkError::JsonError(format!(
                "Expected source json to be an object {}",
                &source
            ))),
        }
    }
    fn marker() -> &'static str {
        "artist"
    }
}

#[allow(unused_imports)]
#[allow(dead_code)]

mod tests {
    use crate::resources::artist::Artist;
    use crate::resources::Resource;
    use serde_json;
    use serde_json::Value;
    use std::fs::File;
    use std::io::Read;

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
        assert_eq!(
            "http://www.songkick.com/artists/253846-radiohead?utm_source=24619&utm_medium=partner",
            artist.uri
        );
        assert_eq!(253846, artist.id);
        assert_eq!(
            "a74b1b7f-71a5-4011-9441-d0b5e4122711",
            artist.identifiers[0].mbid
        );
    }
}
