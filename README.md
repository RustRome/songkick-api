# SongKick Rust


[SongKick]() API library written in Rust

[![Build Status](https://travis-ci.org/RustRome/songkick-api.svg?branch=master)](https://travis-ci.org/RustRome/songkick-api)
[![Coverage](https://codecov.io/gh/maggiolo00/songkick-api/branch/master/graph/badge.svg)](https://codecov.io/gh/maggiolo00/songkick-api)




##Usage


Add this in your Cargo.toml:

```
[dependencies]
songkick = "0.1.0"
```


... and then this in your crate

```
extern crate songkick;
```



##Example


Fetch Artist Info with SongKick ID
 
 ```rust,
use songkick::{SongKick};
 use songkick::resources::Artist;
use songkick::endpoints::{SkEndpoint};
 let sk = SongKick::new("API_KEY");
//RadioHead ID
let artists : Vec<Artist> = sk.artist.get(253846)
.and_then(|res| Ok(res.collect()))
.expect("Failed to fetch artist with id");
assert_eq!(1,artists.len());
```


Check more examples [here](https://github.com/maggiolo00/songkick-api/tree/master/examples)


