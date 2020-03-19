use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use bson;
use mongodb::{self, error::Result as MongoResult};
use serde::{Deserialize, Serialize};
use std::{env, io};

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
	format!("Hello {}! id: {}", info.1, info.0)
}

#[get("/dbs")]
async fn dbs(mongo: web::Data<mongodb::Client>) -> impl Responder {
	match mongo.list_database_names(None) {
		Ok(dbs) => format!("{:?}", dbs),
		Err(e) => format!("{}", e),
	}
}

#[get("/read/{db}/{col}")]
async fn read_data(
	path: web::Path<(String, String)>,
	mongo: web::Data<mongodb::Client>,
) -> impl Responder {
	let (db_name, col_name) = path.into_inner();
	let docs: Vec<bson::Document> = mongo
		.database(&db_name)
		.collection(&col_name)
		.find(None, None)
		.unwrap()
		.collect::<MongoResult<Vec<bson::Document>>>()
		.unwrap();
	format!("{:?}", docs)
}

#[post("/write/{db}/{col}/{val}")]
async fn write_data(
	path: web::Path<(String, String, String)>,
	mongo: web::Data<mongodb::Client>,
) -> impl Responder {
	let (db_name, col_name, value) = path.into_inner();
	let res = mongo
		.database(&db_name)
		.collection(&col_name)
		.insert_one(bson::doc! {"value": value}, None);
	match res {
		Ok(_) => format!("{}", "success!"),
		Err(e) => format!("error: {}", e),
	}
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

fn main() {
	let mongo_client = mongodb::Client::with_uri_str(
		"mongodb+srv://ryan:<pass>@wedding-8rska.mongodb.net/test?retryWrites=true&w=majority",
	)
	.unwrap();
	inner_main(mongo_client).unwrap();
}

#[actix_rt::main]
async fn inner_main(mc: mongodb::Client) -> io::Result<()> {
	env::set_var("RUST_LOG", "actix_web=debug");
	env_logger::init();

	let mongo_client_data = web::Data::new(mc);

	HttpServer::new(move || {
		App::new()
			.app_data(mongo_client_data.clone())
			.wrap(middleware::Logger::default())
			.service(index)
			.service(get_object)
			.service(post_object)
			.service(dbs)
			.service(write_data)
			.service(read_data)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
