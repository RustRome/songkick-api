extern crate songkick;
#[macro_use] extern crate prettytable;
extern crate shrust;

use std::io::{stdin, stdout};
use songkick::{SongKick};
use songkick::resources::Artist;
use songkick::endpoints::{SkEndpoint};
use songkick::resources::{Event};
use songkick::options::{OptionsBuilder};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

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


    let artists : Vec<Artist> = sk.artist.get(253846)
        .and_then(|res| Ok(res.collect()))
        .expect("Failed to fetch artist with id");

    assert_eq!(1,artists.len());

    let res = sk.artist.gigography(253846, None);

    match res {
        Ok(result) => {
            let per_page = 50;

            let total = result.total_entries as f64;
            let p_page = per_page as f64;
            let pages = (total / p_page).ceil() as u64;

            let mut events = result.collect::<Vec<Event>>();

            println!("Fetched first page");
            println!("");
            for n in 2..(pages + 1) {
                let options = OptionsBuilder::new().paging(n, per_page).build();
                println!("Fetching page {} of {} ", n, pages);
                let res = sk.artist.gigography(253846, Some(options)).unwrap();
                let mut evts = res.collect::<Vec<Event>>();
                events.append(&mut evts);
                println!("Fetched page {} of {} ", n, pages);
                println!("");
                sleep(Duration::from_secs(1));
            }
            println!("");
            println!("Events Collected : {}", events.len());
        },
        Err(ex) => {
            println!("Error : {}", ex);
        }
    }
}

