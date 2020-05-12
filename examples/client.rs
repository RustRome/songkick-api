use prettytable::Cell;
use prettytable::Row;
use prettytable::Table;
use prettytable::{cell, row};
use shrust::{Shell, ShellIO};
use songkick::endpoints::SkEndpoint;
use songkick::resources::{Artist, Event};
use songkick::SongKick;
use std::io::Write;
use std::io::{stdin, stdout};

#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_assignments)]
fn main() {
    let welcome = r#"
  _________                      ____  __.__        __     __________                __
 /   _____/ ____   ____    ____ |    |/ _|__| ____ |  | __ \______   \__ __  _______/  |_
 \_____  \ /  _ \ /    \  / ___\|      < |  |/ ___\|  |/ /  |       _/  |  \/  ___/\   __\
 /        (  <_> )   |  \/ /_/  >    |  \|  \  \___|    <   |    |   \  |  /\___ \  |  |
/_______  /\____/|___|  /\___  /|____|__ \__|\___  >__|_ \  |____|_  /____//____  > |__|
        \/            \//_____/         \/       \/     \/         \/           \/
    "#;

    println!("{}", welcome);
    stdout().flush().unwrap();

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

    let mut shell = Shell::new(sk);

    shell.new_command("search", "search api", 2, |io, sk, s| {
        writeln!(io, "Searching {} with {}", s[0], s[1])?;
        match s[0] {
            "artist" => search_artist(sk, &s[1]),
            _ => writeln!(io, "Command {} not found", s[0])?,
        }
        Ok(())
    });

    shell.new_command("calendar", "Calendar API", 2, |io, sk, s| {
        writeln!(io, "Searching  {} events for with {}", s[0], s[1])?;

        let id = s[1].parse::<u64>()?;

        match s[0] {
            "artist" => calendar_artist(sk, id),
            _ => writeln!(io, "Command {} not found", s[0])?,
        }
        Ok(())
    });

    shell.new_command("gigography", "Gigography API", 2, |io, sk, s| {
        writeln!(io, "Searching {} gigs with {}", s[0], s[1])?;

        let id = s[1].parse::<u64>()?;

        match s[0] {
            "artist" => gigs_artist(sk, id),
            _ => writeln!(io, "Command {} not found", s[0])?,
        }
        Ok(())
    });

    shell.new_command("show", "Single Resource API", 2, |io, sk, s| {
        writeln!(io, "Searching {} entity with {}", s[0], s[1])?;

        let id = s[1].parse::<u64>()?;

        match s[0] {
            "artist" => get_artist(sk, id),
            _ => writeln!(io, "Command {} not found", s[0])?,
        };
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}

fn get_artist(sk: &SongKick, input: u64) {
    let res = sk.artist.get(input);

    match res {
        Ok(result) => {
            let artists = result.collect::<Vec<Artist>>();
            if artists.len() == 1 {
                println!("");
                let mut table = Table::new();
                let ref artist = artists[0];
                table.add_row(row!["ID", artist.id]);
                table.add_row(row!["NAME", artist.display_name]);
                table.add_row(row!["URI", artist.uri]);
                table.printstd();
            } else {
                println!("Artist with id {} not found", input);
            }
        }
        Err(ex) => {
            println!("Error : {}", ex);
        }
    }
}

fn gigs_artist(sk: &SongKick, input: u64) {
    let res = sk.artist.gigography(input, None);

    match res {
        Ok(result) => {
            let page = result.page;
            let per_page = result.per_page;
            let total = result.total_entries;

            let events = result.collect::<Vec<Event>>();

            println!("");

            let mut table = Table::new();
            table.add_row(row!["ID", "NAME"]);
            for event in events {
                table.add_row(Row::new(vec![
                    Cell::new(&event.id.to_string()),
                    Cell::new(&event.display_name),
                ]));
            }

            table.printstd();
            println!("");
            println!(
                "Total : {}, Page : {}, Per Page : {}",
                total, page, per_page
            );
            println!("");
        }
        Err(ex) => {
            println!("Error : {}", ex);
        }
    }
}

fn calendar_artist(sk: &SongKick, input: u64) {
    let res = sk.artist.calendar(input, None);

    match res {
        Ok(result) => {
            let page = result.page;
            let per_page = result.per_page;
            let total = result.total_entries;

            let events = result.collect::<Vec<Event>>();

            println!("");

            let mut table = Table::new();
            table.add_row(row!["ID", "NAME"]);
            for event in events {
                table.add_row(Row::new(vec![
                    Cell::new(&event.id.to_string()),
                    Cell::new(&event.display_name),
                ]));
            }

            table.printstd();
            println!("");
            println!(
                "Total : {}, Page : {}, Per Page : {}",
                total, page, per_page
            );
            println!("");
        }
        Err(ex) => {
            println!("Error : {}", ex);
        }
    }
}

fn search_artist(sk: &SongKick, input: &str) {
    let res = sk.artist.search_by_name(input);

    match res {
        Ok(result) => {
            let page = result.page;
            let per_page = result.per_page;
            let total = result.total_entries;

            let artists = result.collect::<Vec<Artist>>();

            let mut table = Table::new();

            println!("");
            table.add_row(row!["ID", "NAME"]);

            for artist in artists {
                table.add_row(Row::new(vec![
                    Cell::new(&artist.id.to_string()),
                    Cell::new(&artist.display_name),
                ]));
            }

            table.printstd();
            println!("");
            println!(
                "Total : {}, Page : {}, Per Page : {}",
                total, page, per_page
            );
            println!("");
        }
        Err(ex) => {
            println!("Error : {}", ex);
        }
    }
}
