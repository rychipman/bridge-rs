use super::bidding;
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
        .subcommand(
            SubCommand::with_name("bid")
                .about("Prompt the user to make a bid in the provided scenario")
                .arg(
                    Arg::with_name("opening")
                        .short("o")
                        .long("opening")
                        .help("create and bid an opening hand"),
                ),
        )
        .subcommand(
            SubCommand::with_name("login")
                .about("Logs in with the provided email address")
                .arg(
                    Arg::with_name("email")
                        .help("the email address to use for login")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("logout").about("Logs out the current user, if logged in"),
        )
        .subcommand(
            SubCommand::with_name("register")
                .about("Creates a new user with the provided email address")
                .arg(
                    Arg::with_name("email")
                        .help("the email address to use for registration")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("user")
                .about("Prints information about the currently logged-in user"),
        )
        .get_matches();

    match matches.subcommand() {
        ("bid", Some(m)) => run_bid(m),
        ("login", Some(m)) => run_login(m),
        ("logout", Some(m)) => run_logout(m),
        ("register", Some(m)) => run_register(m),
        ("user", Some(m)) => run_user(m),
        _ => panic!("unknown subcommand"),
    }
}

fn run_bid(matches: &ArgMatches) {
    if matches.is_present("opening") {
        bidding::bid_opening()
    } else {
        bidding::bid_continuation()
    }
}

fn run_login(matches: &ArgMatches) {
    let email = matches.value_of("email").unwrap();
    bidding::login(email);
}

fn run_logout(_matches: &ArgMatches) {
    bidding::logout();
}

fn run_register(matches: &ArgMatches) {
    let email = matches.value_of("email").unwrap();
    bidding::register(email);
}

fn run_user(_matches: &ArgMatches) {
    match bidding::current_user() {
        Some(u) => println!("current user: {:?}", u),
        None => println!("no user logged in"),
    };
}
