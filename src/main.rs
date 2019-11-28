extern crate clap;
extern crate phf;
extern crate rand;
extern crate rust_cowsay;

use clap::{App, Arg};
use rand::seq::SliceRandom;
use rust_cowsay::{format_cow, get_eyes, list_cows};
use std::io::{self, Read};

fn main() {
    let matches = App::new("rust-cowsay")
        .version("v0.1.0-pre-alpha")
        .author("Matt Smith. <matthew.smith491@gmail.com>")
        .arg(
            Arg::with_name("MESSAGE")
                .help("Message for cow to say")
                .multiple(true),
        )
        .arg(
            Arg::with_name("cow")
                .short("f")
                .value_name("COW")
                .help("Which cow should say"),
        )
        .arg(Arg::with_name("think").short("t").help("Think"))
        .arg(
            Arg::with_name("width")
                .short("W")
                .value_name("WIDTH")
                .help("Max width of cow text bubble"),
        )
        .arg(
            Arg::with_name("nowrap")
                .short("n")
                .long("nowrap")
                .help("Disable word wrap"),
        )
        .arg(
            Arg::with_name("eyes")
                .short("e")
                .value_name("EYES")
                .help("Which eyes to pick or provide custom ones"),
        )
        .arg(
            Arg::with_name("tongue")
                .short("T")
                .value_name("TONGUE_STRING")
                .help("Custom Tongue"),
        )
        .arg(
            Arg::with_name("random")
                .short("r")
                .long("random")
                .help("Choose random cow"),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("print all the cows"),
        )
        .get_matches();

    let mut cow = matches.value_of("cow").unwrap_or("default").to_owned();

    if matches.is_present("random") {
        let cows = list_cows();
        cow = cows.choose(&mut rand::thread_rng()).unwrap().to_owned();
    }

    let width = matches
        .value_of("width")
        .unwrap_or("40")
        .parse::<usize>()
        .unwrap();

    let wrap = !matches.is_present("nowrap");
    let message_vals = match matches.values_of("MESSAGE") {
        Some(x) => x.collect::<Vec<_>>(),
        None => vec![""],
    };
    let mut message = message_vals.join(" ");

    message = match &message[..] {
        "" => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer.trim_end().to_string()
        }
        _ => message,
    };

    let tongue = matches.value_of("tongue").unwrap_or(" ");

    // Cow Eyes
    let custom_eyes = matches.value_of("eyes").unwrap_or("default");
    let eyes = get_eyes(custom_eyes);

    let think = matches.is_present("think");

    if matches.is_present("all") {
        let list = list_cows();
        for c in list {
            let formatted_cow = format_cow(&message, &c, width, think, wrap, eyes, tongue);
            println!("{}", c);
            println!("{}", formatted_cow);
        }
    } else {
        let formatted_cow = format_cow(&message, &cow, width, think, wrap, eyes, tongue);
        println!("{}", formatted_cow);
    }
}
