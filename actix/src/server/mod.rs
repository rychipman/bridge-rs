use actix_web::{middleware, web, App, HttpServer};
use std::io;

mod auth;
mod routes;

pub struct Config {
	pub mongo_client: mongodb::Client,
	pub addr: String,
}

#[actix_rt::main]
pub async fn run(cfg: Config) -> io::Result<()> {
	let mongo_client_data = web::Data::new(cfg.mongo_client);

	HttpServer::new(move || {
		App::new()
			.app_data(mongo_client_data.clone())
			.wrap(middleware::Logger::default())
			.configure(routes::account::config)
			.configure(routes::exercise::config)
			.configure(routes::test::config)
	})
	.bind(&cfg.addr)?
	.run()
	.await
}
