use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{env, io};

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
	format!("Hello {}! id: {}", info.1, info.0)
}

#[derive(Debug, Serialize, Deserialize)]
struct MyObject {
	foo: String,
	bar: String,
}

#[get("/object")]
async fn get_object() -> impl Responder {
	let obj = MyObject {
		foo: "fooval".into(),
		bar: "barval".into(),
	};
	HttpResponse::Ok().json(obj)
}

#[post("/putobj")]
async fn post_object(body: web::Json<MyObject>) -> impl Responder {
	format!("Hello {:?}!", body)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
	env::set_var("RUST_LOG", "actix_web=debug");
	env_logger::init();

	HttpServer::new(|| {
		App::new()
			.wrap(middleware::Logger::default())
			.service(index)
			.service(get_object)
			.service(post_object)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
