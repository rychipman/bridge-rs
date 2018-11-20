use super::web;
use clap::{App, Arg, SubCommand};

pub fn run() {
    let matches = App::new("Bridge CLI")
        .version("0.1")
        .author("Ryan Chipman <ryan@ryanchipman.com>")
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("database")
                .help("specify the sqlite db file to use")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("set the output verbosity"),
        )
        .subcommand(SubCommand::with_name("deal").about("Generates bridge hands"))
        .subcommand(
            SubCommand::with_name("server")
                .about("Runs the web server for collaborative bidding practice"),
        )
        .get_matches();

    let db_file = matches.value_of("database").unwrap_or("bridge.sqlite");
    println!("database file: {}", db_file);

    let verbosity = match matches.occurrences_of("v") {
        0 => "quiet",
        1 => "level one",
        2 => "level two",
        _ => "other",
    };
    println!("verbosity: {}", verbosity);

    if let Some(matches) = matches.subcommand_matches("deal") {
        println!("deal subcommand not implemented yet");
    }

    if let Some(matches) = matches.subcommand_matches("server") {
        println!("running bridge server...");
        web::rocket().launch();
    }
}
