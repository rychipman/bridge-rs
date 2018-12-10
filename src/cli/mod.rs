use super::{bidding, web};
use clap::{App, Arg, ArgMatches, SubCommand};

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
            SubCommand::with_name("user")
                .about("Does some user-management stuff")
                .arg(
                    Arg::with_name("email")
                        .help("the email address of the user in question")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("Runs the web server for collaborative bidding practice"),
        )
        .get_matches();

    match matches.subcommand() {
        ("deal", Some(m)) => run_deal(m),
        ("server", Some(m)) => run_server(m),
        ("user", Some(m)) => run_user(m),
        _ => panic!("unknown subcommand"),
    }
}

fn run_deal(_matches: &ArgMatches) {
    bidding::play_arbitrary_exercise();
    bidding::show_exercises_with_bids();
}

fn run_server(_matches: &ArgMatches) {
    println!("running bridge server...");
    web::rocket().launch();
}

fn run_user(matches: &ArgMatches) {
    let email = matches.value_of("email").unwrap();
    bidding::find_or_create_user(email.to_string());
}
