use SkResult;
use resources::Resource;
use serde_json::Value;
use error::SkError;
use resources::venue::Venue;
use resources::artist::Artist;
use util::json::{get_str, get_u64, get_f64, get_arr};

#[derive(Debug, PartialEq)]


pub struct When {
    pub datetime: Option<String>,
    pub time: Option<String>,
    pub date: Option<String>
}

pub struct Performance {
    pub billing: String,
    pub billing_index: u64,
    pub id: u64,
    pub display_name: String,
    pub artist: Artist,
}

impl Resource for Performance {
    fn from_json(source: &Value) -> SkResult<Self> where Self: Sized {
        match source.as_object() {
            Some(obj) => {
                let display_name = try!(get_str(obj, "displayName"));
                let billing = try!(get_str(obj, "billing"));
                let id = try!(get_u64(obj, "id"));
                let billing_index = try!(get_u64(obj, "billingIndex"));
                let artist = obj.get("artist").unwrap();

                let artist = try!(Artist::from_json(&artist));

                Ok(Performance {
                    id: id,
                    display_name: display_name,
                    billing: billing,
                    billing_index: billing_index,
                    artist: artist
                })
            },
            None => Err(SkError::JsonError(format!("Expected source json to be an object {}", &source)))
        }
    }

    fn marker() -> &'static str {
        unimplemented!()
    }
}

impl Resource for When {
    fn from_json(source: &Value) -> SkResult<When> where Self: Sized {
        match source.as_object() {
            Some(obj) => {
                let datetime = match obj.get("datetime") {
                    Some(datetime) => {
                        if datetime.is_null() {
                            None
                        } else {
                            Some(String::from(datetime.as_str().unwrap()))
                        }
                    },
                    None => None
                };
                let time = match obj.get("time") {
                    Some(time) => {
                        if time.is_null() {
                            None
                        } else {
                            Some(String::from(time.as_str().unwrap()))
                        }
                    },
                    None => None
                };

                let date = match obj.get("date") {
                    Some(date) => {
                        if date.is_null() {
                            None
                        } else {
                            Some(String::from(date.as_str().unwrap()))
                        }
                    },
                    None => None
                };


                Ok(When {
                    date: date,
                    datetime: datetime,
                    time: time,
                })
            }
            None => Err(SkError::JsonError(format!("Expected source json to be an object {}", &source)))
        }
    }

    fn marker() -> &'static str {
        unimplemented!()
    }
}

// Event Resource
pub struct Event {
    pub id: u64,
    pub event_type: String,
    pub display_name: String,
    pub status: String,
    pub uri: String,
    pub popularity: f64,
    pub venue: Venue,
    pub start: When,
    pub end: Option<When>,
    pub performances: Vec<Performance>
}


impl Resource for Event {
    fn from_json(source: &Value) -> SkResult<Self> where Self: Sized {
        match source.as_object() {
            Some(obj) => {
                let id = try!(get_u64(obj, "id"));
                let display_name = try!(get_str(obj, "displayName"));

                let uri = try!(get_str(obj, "uri"));
                let event_type = try!(get_str(obj, "type"));

                let status = try!(get_str(obj, "status"));
                let popularity = try!(get_f64(obj, "popularity"));


                let start = obj.get("start").unwrap();
                let start = try!(When::from_json(&start));

                let mut end = None;
                if let Some(ref e) = obj.get("end") {
                    end = Some(try!(When::from_json(&e)));
                }

                let venue = obj.get("venue").unwrap();
                let venue = try!(Venue::from_json(&venue));

                let mut performances = Vec::new();
                let performance = try!(get_arr(obj, "performance"));

                for p in performance {
                    let model = try!(Performance::from_json(&p));
                    performances.push(model);
                }

                Ok(Event {
                    id: id,
                    event_type: event_type,
                    display_name: display_name,
                    status: status,
                    uri: uri,
                    popularity: popularity,
                    venue: venue,
                    start: start,
                    end: end,
                    performances: performances
                })
            },
            None => Err(SkError::JsonError(format!("Expected source json to be an object {}", &source)))
        }
    }

    fn marker() -> &'static str {
        "event"
    }
}




#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use resources::event::Event;
    use resources::event::Performance;
    use resources::{Resource};
    use serde_json::Value;
    use serde_json;

    fn load_event(path: &str) -> Event {
        let sample_str = {
            let mut file = File::open(path).unwrap();
            let mut ret = String::new();
            file.read_to_string(&mut ret).unwrap();
            ret
        };
        let data: Value = serde_json::from_str(&sample_str).unwrap();
        Event::from_json(&data).unwrap()
    }


    #[test]
    fn test_event_json() {
        let event = load_event("fixtures/event/single-event-artist-324967.json");


        assert_eq!("Placebo with The Mirror Trap at Sentrum Scene (October 16, 2016)", event.display_name);
        assert_eq!("http://www.songkick.com/concerts/26486224-placebo-at-sentrum-scene?utm_source=24619&utm_medium=partner", event.uri);
        assert_eq!(26486224, event.id);
        assert_eq!("ok", event.status);
        assert_eq!("Concert", event.event_type);

        assert_eq!(0.156595, event.popularity);


        assert_eq!(Some(String::from("2016-10-16T19:00:00+0000")), event.start.datetime);
        // Assert Venue
        assert_eq!(Some(String::from("Sentrum Scene")), event.venue.display_name);
        assert_eq!(Some(33495), event.venue.id);
        assert_eq!(Some(59.9155184), event.venue.lat);
        assert_eq!(Some(10.7518051), event.venue.lng);
        assert_eq!(Some(String::from("http://www.songkick.com/venues/33495-sentrum-scene?utm_source=24619&utm_medium=partner")), event.venue.uri);

        // Assert Metro Area

        let metro = event.venue.metro_area.unwrap();
        assert_eq!("Oslo", metro.display_name);
        assert_eq!(31422, metro.id);
        assert_eq!("http://www.songkick.com/metro_areas/31422-norway-oslo?utm_source=24619&utm_medium=partner", metro.uri);
        assert_eq!("Norway", metro.country.display_name);

        // Assert Performances

        assert_eq!(2, event.performances.len());

        let ref performance : Performance = event.performances[0];
        assert_eq!(51721634, performance.id);
        assert_eq!("Placebo", performance.display_name);
        assert_eq!("headline", performance.billing);
        assert_eq!(1, performance.billing_index);

        assert_eq!("The Mirror Trap", event.performances[1].display_name);
        assert_eq!("support", event.performances[1].billing);
        assert_eq!(2, event.performances[1].billing_index);
    }
}