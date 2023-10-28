extern crate clap;
extern crate phf;
extern crate rand;
extern crate cowsay;

use clap::{Command, Arg};
use rand::seq::SliceRandom;
use cowsay::{format_cow, get_eyes, list_cows};
use std::io::{self, Read};

fn main() {
    let matches = Command::new("cowsay")
    .version("v0.1.5")
    .author("Jose Perez <28468662+ThisNekoGuy@users.noreply.github.com>, duck <113956421+duckfromdiscord@users.noreply.github.com>")
    .arg(
        Arg::new("MESSAGE")
        .help("Message for cow to say")
    )
    .arg(
        Arg::new("cow")
        .short('f')
        .value_name("COW")
        .help("Which cow should say")
    )
    .arg(
        Arg::new("think")
        .short('t')
        .help("Think")
        .num_args(0)
    )
    .arg(
        Arg::new("width")
        .short('W')
        .value_name("WIDTH")
        .help("Max width of cow text bubble")
    )
    .arg(
        Arg::new("nowrap")
        .short('n')
        .long("nowrap")
        .help("Disable word wrap")
        .num_args(0)
    )
    .arg(
        Arg::new("eyes")
        .short('e')
        .long("EYES")
        .help("Which eyes to pick or provide custom ones")
    )
    .arg(
        Arg::new("tongue")
        .short('T')
        .long("TONGUE_STRING")
        .help("Custom Tongue")
    )
    .arg(
        Arg::new("random")
        .short('r')
        .long("random")
        .help("Choose random cow")
        .num_args(0)
    )
    .arg(
        Arg::new("all")
        .short('a')
        .long("all")
        .help("print all the cows")
        .num_args(0)
    )
    .get_matches();

    let width = matches
        .get_one::<String>("width")
        .map(|x| &**x)
        .unwrap_or("40")
        .parse::<usize>()
        .unwrap();

    let wrap = !matches.get_flag("nowrap");
    let message_vals: Vec<&str> = match matches.get_many::<String>("MESSAGE") {
        Some(x) => x.map(|x| &**x).collect::<Vec<_>>(),
        None => vec![""],
    };
    let mut message = message_vals.join(" ");

    if message.is_empty(){
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        message = buffer.trim_end().to_string();
    }


    let tongue = matches.get_one::<String>("tongue").map(|x| &**x).unwrap_or(" ");
    let custom_eyes = matches.get_one::<String>("eyes").map(|x| &**x).unwrap_or("default");
    let eyes = get_eyes(custom_eyes);

    let think = matches.get_flag("think");

    if matches.get_flag("all") {
        let list = list_cows();
        for cow in list {
            let formatted_cow = format_cow(&message, &cow, width, think, wrap, eyes, tongue);
            println!("{}", cow);
            println!("{}", formatted_cow);
        }
    } else {
        let cow = if matches.get_flag("random") {
            let cows = list_cows();
            cows.choose(&mut rand::thread_rng()).unwrap().to_owned()
        }else{
            matches.get_one::<String>("cow").map(|x| &**x).unwrap_or("default").to_owned()
        };
        let formatted_cow = format_cow(&message, &cow, width, think, wrap, eyes, tongue);
        println!("{}", formatted_cow);
    }
}
