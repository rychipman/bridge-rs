use dotenv::dotenv;
use mongodb;
use std::env;

mod db;
mod result;
mod server;

fn main() {
	dotenv().ok();
	env_logger::init();

	let mongo_uri = env::var("MONGO_URI").expect("no MONGO_URI var set");
	let mongo_client =
		mongodb::Client::with_uri_str(&mongo_uri).expect("failed to create mongo client");

	let cfg = server::Config { mongo_client };
	server::run(cfg).expect("server run failed");
}
