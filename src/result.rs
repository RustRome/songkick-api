use crate::error::SkError;
use crate::resources::Resource;
use crate::SkResult;
use serde_json::Value;
use std::vec::IntoIter;

/// Struct for handling response from API calls
pub struct SkResultSet<M: Resource> {
    /// Status of the request
    pub status: String,
    iter: IntoIter<M>,
    /// Current Page
    pub page: u64,
    /// Items per Page
    pub per_page: u64,
    /// Total Entries
    pub total_entries: u64,
}

impl<M> SkResultSet<M>
where
    M: Resource,
{
    #[doc(hidden)]
    pub fn from_json(source: &Value) -> SkResult<SkResultSet<M>> {
        let obj = source
            .as_object()
            .unwrap()
            .get("resultsPage")
            .unwrap()
            .as_object()
            .unwrap();

        let status = String::from(obj.get("status").unwrap().as_str().unwrap());

        if status == "error" {
            let error = obj.get("error").unwrap().as_object().unwrap();

            let message = error.get("message").unwrap().as_str().unwrap();

            return Err(SkError::BadRequest(String::from(message)));
        }

        let mut page = 1;
        let mut per_page = 50;
        let mut total_entries = 1;

        if let Some(ref p) = obj.get("page") {
            page = p.as_u64().unwrap();
        }
        if let Some(ref p) = obj.get("perPage") {
            per_page = p.as_u64().unwrap();
        }

        if let Some(ref p) = obj.get("totalEntries") {
            total_entries = p.as_u64().unwrap();
        }

        let result = obj.get("results").unwrap().as_object().unwrap();

        let mut results: Vec<M> = Vec::new();

        if let Some(ref r) = result.get(M::marker()) {
            if r.is_object() {
                let model = M::from_json(&r)?;
                results.push(model)
            } else if r.is_array() {
                for res in r.as_array().unwrap() {
                    let model = M::from_json(&res)?;
                    results.push(model);
                }
            }
        }
        Ok(SkResultSet {
            iter: results.into_iter(),
            status: status,
            page: page,
            per_page: per_page,
            total_entries: total_entries,
        })
    }
}

impl<M> Iterator for SkResultSet<M>
where
    M: Resource,
{
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[allow(unused_imports)]
#[allow(dead_code)]
mod tests {
    use crate::error::SkError;
    use crate::resources::artist::Artist;
    use crate::resources::event::Event;
    use crate::resources::event::When;
    use crate::resources::Resource;
    use crate::result::SkResultSet;
    use serde_json;
    use serde_json::Value;
    use std::fs::File;
    use std::io::Read;
    use crate::SkResult;

    fn load_result<M: Resource>(path: &str) -> SkResult<SkResultSet<M>> {
        let sample_str = {
            let mut file = File::open(path).unwrap();
            let mut ret = String::new();
            file.read_to_string(&mut ret).unwrap();
            ret
        };
        let data: Value = serde_json::from_str(&sample_str).unwrap();
        SkResultSet::from_json(&data)
    }

    #[test]
    fn single_result_artist_test() {
        let mut res = load_result::<Artist>("fixtures/artist/single-artist-324967.json").unwrap();

        assert_eq!("ok", res.status);

        assert_eq!(1, res.total_entries);
        assert_eq!(1, res.page);
        assert_eq!(50, res.per_page);

        let artist = res.next();

        assert_eq!(artist.is_none(), false);

        let artist = artist.unwrap();

        assert_eq!(artist.display_name, "Placebo");
        assert_eq!(artist.id, 324967);

        assert_eq!(
            artist.identifiers[0].mbid,
            "81b9963b-7ff7-47f7-9afb-fe454d8db43c"
        );
    }

    #[test]
    fn search_result_artist_test() {
        let res = load_result::<Artist>("fixtures/artist/artist-search-placebo.json").unwrap();

        assert_eq!("ok", res.status);

        assert_eq!(10, res.total_entries);
        assert_eq!(1, res.page);
        assert_eq!(50, res.per_page);

        let artists = res.collect::<Vec<Artist>>();

        assert_eq!(artists.len(), 10);

        assert_eq!(artists[0].display_name, "Placebo");
        assert_eq!(artists[0].id, 324967);

        assert_eq!(
            artists[0].identifiers[0].mbid,
            "81b9963b-7ff7-47f7-9afb-fe454d8db43c"
        );
    }

    #[test]
    fn search_result_event_for_artist_test() {
        let res = load_result::<Event>("fixtures/event/artist-324967-calendar.json").unwrap();

        assert_eq!("ok", res.status);

        assert_eq!(33, res.total_entries);
        assert_eq!(1, res.page);
        assert_eq!(50, res.per_page);

        let events = res.collect::<Vec<Event>>();

        assert_eq!(events.len(), 33);

        assert_eq!(
            "Placebo with The Mirror Trap at Cirkus (October 18, 2016)",
            events[0].display_name
        );
        assert_eq!(26486294, events[0].id);

        match events[0].venue.metro_area {
            Some(ref m) => {
                assert_eq!("Stockholm", m.display_name);
            }
            None => assert!(false),
        }

        assert_eq!(
            When {
                datetime: Some(String::from("2016-10-18T19:30:00+0200")),
                time: Some(String::from("19:30:00")),
                date: Some(String::from("2016-10-18"))
            },
            events[0].start
        );
        assert_eq!(None, events[0].end);
    }

    #[test]
    fn single_event_get_by_id() {
        let res =
            load_result::<Event>("fixtures/event/single-event-festival-27081999.json").unwrap();

        assert_eq!("ok", res.status);
        assert_eq!(1, res.total_entries);
        assert_eq!(1, res.page);
        assert_eq!(50, res.per_page);

        let events = res.collect::<Vec<Event>>();

        assert_eq!(events.len(), 1);

        assert_eq!(
            "Pitchfork Music Festival Paris 2016",
            events[0].display_name
        );
        assert_eq!("Festival", events[0].event_type);
        assert_eq!(27081999, events[0].id);

        match events[0].venue.metro_area {
            Some(ref m) => {
                assert_eq!("Paris", m.display_name);
            }
            None => assert!(false),
        }

        assert_eq!(
            Some(String::from("Grande Halle de la Villette")),
            events[0].venue.display_name
        );

        assert_eq!(
            Some(String::from("2016-10-27T17:00:00+0200")),
            events[0].start.datetime
        );
        assert_eq!(Some(String::from("2016-10-27")), events[0].start.date);
        assert_eq!(Some(String::from("17:00:00")), events[0].start.time);

        assert_eq!(
            Some(When {
                datetime: None,
                time: None,
                date: Some(String::from("2016-10-29"))
            }),
            events[0].end
        );
    }

    #[test]
    fn empty_sarch() {
        let res = load_result::<Artist>("fixtures/empty_search.json").unwrap();

        assert_eq!("ok", res.status);
        assert_eq!(0, res.total_entries);
        assert_eq!(1, res.page);
        assert_eq!(50, res.per_page);

        let artists = res.collect::<Vec<Artist>>();

        assert_eq!(artists.len(), 0);
    }

    #[test]
    fn invalid_api_key() {
        let res = load_result::<Artist>("fixtures/invalid_key.json");

        assert_eq!(true, res.is_err());

        let err = res.err().unwrap();

        match err {
            SkError::BadRequest(ref err) => {
                assert_eq!("Invalid or missing apikey", err);
            }
            _ => assert!(false),
        }
    }
}
