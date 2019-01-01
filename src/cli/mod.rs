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
            SubCommand::with_name("migrate").about("Runs new database migrations, if necessary"),
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
        ("migrate", Some(m)) => run_migrate(m),
        ("register", Some(m)) => run_register(m),
        ("user", Some(m)) => run_user(m),
        _ => panic!("unknown subcommand"),
    }
}

fn run_bid(matches: &ArgMatches) {
    match bidding::bid(matches.is_present("opening")) {
        Ok(()) => println!("finished bidding with no error"),
        Err(e) => println!("encountered error while bidding: {}", e),
    }
}

fn run_login(matches: &ArgMatches) {
    let email = matches.value_of("email").unwrap();
    match bidding::login(email) {
        Ok(_) => println!("login successful"),
        Err(_) => println!("login failed"),
    }
}

fn run_logout(_matches: &ArgMatches) {
    match bidding::logout() {
        Ok(_) => println!("logout successful"),
        Err(_) => println!("logout failed"),
    }
}

fn run_migrate(_matches: &ArgMatches) {
    if let Err(e) = bidding::run_migrations() {
        println!("encountered error while running migrations: {}", e)
    }
}

fn run_register(matches: &ArgMatches) {
    let email = matches.value_of("email").unwrap();
    match bidding::register(email) {
        Ok(_) => println!("registration successful"),
        Err(_) => println!("registration failed"),
    }
}

fn run_user(_matches: &ArgMatches) {
    match bidding::current_user().ok() {
        Some(u) => println!("current user: {:?}", u),
        None => println!("no user logged in"),
    };
}
