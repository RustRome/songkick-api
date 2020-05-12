<h1 align="center">SongKick Rust</h1>
<div align="center">
  <strong>
    SongKick API library written in Rust
  </strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/RustRome/songkick-api/actions?query=workflow%3ATests">
    <img src="https://github.com/RustRome/songkick-api/workflows/Tests/badge.svg"
    alt="Tests status" />
  </a>
  
  <a href='https://coveralls.io/github/RustRome/songkick-api?branch=master'>
    <img src='https://coveralls.io/repos/github/RustRome/songkick-api/badge.svg?branch=master' alt='Coverage Status' />
  </a>

  <a href="https://crates.io/crates/songkick-api">
    <img src="https://img.shields.io/crates/d/songkick-api.svg?style=flat-square"
      alt="Download" />
  </a>

  <a href="https://docs.rs/songkick-api">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>

</div>

<br/>




## Usage


Add this in your Cargo.toml:

```
[dependencies]
songkick = "0.1.0"
```


... and then this in your crate

```
extern crate songkick;
```



## Example


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


Check more examples [here](https://github.com/RustRome/songkick-api/tree/master/examples)


