extern crate songkick;
extern crate prettytable;
extern crate shrust;

use std::io::{stdin, stdout};
use songkick::{SongKick};
use songkick::resources::{Event};
use songkick::options::{OptionsBuilder};
use std::io::Write;

#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_assignments)]
fn main() {

    let mut api_key: Option<String> = None;
    loop {
        print!("* Insert an api key: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        let res = stdin().read_line(&mut input);
        if res.is_ok() {
            api_key = Some(input);
            break;
        }
    }
    let sk = SongKick::new(api_key.unwrap());

    let option = OptionsBuilder::new().filter(|f| {
        f.artist_name("Radiohead")
            .location("clientip");
    }).build();
    let events: Vec<Event> = sk.event.search(option)
        .and_then(|res| Ok(res.collect()))
        .expect("Failed to search local Radiohead concerts");

    println!("");
    println!("Found {} Radiohead Events near you", events.len());
    println!("");
}

