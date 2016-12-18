use util::encode;

pub struct Options {
    paging: Option<Paging>,
    filter: Option<Filter>,
    sort: Option<Sort>
}


struct Filter {
    artist_name: Option<String>,
    min_date: Option<String>,
    max_date: Option<String>,
    location: Option<String>
}

pub enum Sort {
    ASC,
    DESC
}


struct Paging {
    per_page: u64,
    page: u64
}


pub struct FilterBuilder {
    empty: bool,
    artist_name: Option<String>,
    min_date: Option<String>,
    max_date: Option<String>,
    location: Option<String>
}

impl FilterBuilder {
    fn new() -> FilterBuilder {
        FilterBuilder {
            empty: true,
            artist_name: None,
            min_date: None,
            max_date: None,
            location: None,
        }
    }


    fn artist_name(&mut self, name: String) -> &mut FilterBuilder {
        self.empty = false;
        self.artist_name = Some(name);
        self
    }

    fn min_date(&mut self, min_date: String) -> &mut FilterBuilder {
        self.empty = false;
        self.min_date = Some(min_date);
        self
    }
    fn max_date(&mut self, max_date: String) -> &mut FilterBuilder {
        self.empty = false;
        self.max_date = Some(max_date);
        self
    }

    fn location(&mut self, location: String) -> &mut FilterBuilder {
        self.empty = false;
        self.location = Some(location);
        self
    }

    fn build(self) -> Option<Filter> {
        match self.empty {
            false => {
                Some(Filter {
                    max_date: self.max_date,
                    min_date: self.min_date,
                    artist_name: self.artist_name,
                    location: self.location
                })
            },
            true => None
        }
    }
}

pub struct OptionsBuilder {
    filter: FilterBuilder,
    paging: Option<Paging>,
    sort: Option<Sort>
}


impl OptionsBuilder {
    pub fn new() -> OptionsBuilder {
        OptionsBuilder {
            paging: None,
            filter: FilterBuilder::new(),
            sort: None,
        }
    }

    pub fn paging(mut self, page: u64, per_page: u64) -> OptionsBuilder {
        self.paging = Some(Paging {
            per_page: per_page,
            page: page
        });
        self
    }
    pub fn sort(mut self, sort: Sort) -> OptionsBuilder {
        self.sort = Some(sort);
        self
    }

    pub fn filter<F>(mut self, filter: F) -> OptionsBuilder where F: Fn(&mut FilterBuilder) {
        filter(&mut self.filter);
        self
    }
    pub fn build(self) -> Options {
        Options {
            paging: self.paging,
            filter: self.filter.build(),
            sort: self.sort,
        }
    }
}


pub fn format_with_options(url: &str, options: Option<Options>) -> String {
    match options {
        Some(opts) => {
            let mut new_url = String::from(url);


            // filtering

            if let Some(filter) = opts.filter {
                if let Some(min_date) = filter.min_date {
                    new_url = format!("{}&min_date={}", new_url, encode(&min_date));
                }
                if let Some(max_date) = filter.max_date {
                    new_url = format!("{}&max_date={}", new_url, encode(&max_date));
                }
                if let Some(artist_name) = filter.artist_name {
                    new_url = format!("{}&artist_name={}", new_url, encode(&artist_name));
                }
                if let Some(location) = filter.location {
                    new_url = format!("{}&location={}", new_url, encode(&location));
                }
            }

            // pagination
            if let Some(paging) = opts.paging {
                new_url = format!("{}&page={}", new_url, paging.page);
                new_url = format!("{}&per_page={}", new_url, paging.per_page)
            }

            // sorting

            if let Some(sort) = opts.sort {
                let order = match sort {
                    Sort::ASC => String::from("asc"),
                    Sort::DESC => String::from("desc"),
                };
                new_url = format!("{}&order={}", new_url, order);
            }


            new_url
        }
        None => String::from(url)
    }
}

#[cfg(test)]
mod tests {
    use options::OptionsBuilder;
    use client::SongKickOpts;
    use std::sync::Arc;
    use hyper::*;
    use options::format_with_options;
    use options::Sort;


    #[test]
    fn no_options() {
        let sk = mock_sk_options();

        let url = format!("{}/{}/{}/calendar.json?apikey={}", sk.base_path(), "artists", 253846, sk.api_key());

        assert_eq!("http://api.songkick.com/api/3.0/artists/253846/calendar.json?apikey=DUMMY", format_with_options(&url, None));
    }

    #[test]
    fn artist_calendar_pagination() {
        let sk = mock_sk_options();

        let url = format!("{}/{}/{}/calendar.json?apikey={}", sk.base_path(), "artists", 253846, sk.api_key());

        let options = OptionsBuilder::new().paging(2, 25).build();

        assert_eq!("http://api.songkick.com/api/3.0/artists/253846/calendar.json?apikey=DUMMY&page=2&per_page=25", format_with_options(&url, Some(options)));
    }

    #[test]
    fn artist_calendar_sort() {
        let sk = mock_sk_options();

        let url = format!("{}/{}/{}/calendar.json?apikey={}", sk.base_path(), "artists", 253846, sk.api_key());

        let options = OptionsBuilder::new().sort(Sort::DESC).build();

        assert_eq!("http://api.songkick.com/api/3.0/artists/253846/calendar.json?apikey=DUMMY&order=desc", format_with_options(&url, Some(options)));
    }

    #[test]
    fn artist_calendar_sort_and_pagination() {
        let sk = mock_sk_options();

        let url = format!("{}/{}/{}/calendar.json?apikey={}", sk.base_path(), "artists", 253846, sk.api_key());

        let options = OptionsBuilder::new().paging(2, 25).sort(Sort::DESC).build();

        assert_eq!("http://api.songkick.com/api/3.0/artists/253846/calendar.json?apikey=DUMMY&page=2&per_page=25&order=desc", format_with_options(&url, Some(options)));
    }

    #[test]
    fn event_search_with_location_and_name() {
        let sk = mock_sk_options();

        let url = format!("{}/{}.json?apikey={}", sk.base_path(), "events", sk.api_key());

        let options = OptionsBuilder::new().filter(|f| {
            f.artist_name(String::from("Radiohead")).location(String::from("clientip"));
        }).build();

        let ass = "http://api.songkick.com/api/3.0/events.json?apikey=DUMMY&artist_name=Radiohead&location=clientip";
        assert_eq!(ass, format_with_options(&url, Some(options)));
    }

    #[test]
    fn artist_calendar_filter() {
        let sk = mock_sk_options();

        let url = format!("{}/{}/{}/calendar.json?apikey={}", sk.base_path(), "artists", 253846, sk.api_key());

        let options = OptionsBuilder::new().filter(|f| {
            f.min_date(String::from("2017-06-06")).max_date(String::from("2017-06-09"));
        }).paging(1, 5).sort(Sort::DESC).build();
        assert_eq!("http://api.songkick.com/api/3.0/artists/253846/calendar.json?apikey=DUMMY&min_date=2017%2D06%2D06&max_date=2017%2D06%2D09&page=1&per_page=5&order=desc", format_with_options(&url, Some(options)));
    }


    fn mock_sk_options() -> SongKickOpts {
        let hyper = Arc::new(Client::new());

        SongKickOpts::new(String::from("DUMMY"), hyper, "http://api.songkick.com/api/3.0")
    }
}