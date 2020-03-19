use mongodb;

mod db;
mod result;
mod server;

fn main() {
	let mongo_client = mongodb::Client::with_uri_str(
		"mongodb+srv://ryan:<pass>@wedding-8rska.mongodb.net/test?retryWrites=true&w=majority",
	)
	.expect("failed to create mongo client");
	let cfg = server::Config { mongo_client };
	server::run(cfg).expect("server run failed");
}
