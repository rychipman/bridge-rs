use crate::server::auth;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/test")
			.service(index)
			.service(get_object)
			.service(post_object)
			.service(dbs)
			.service(write_data)
			.service(read_data)
			.service(authed),
	);
}

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
		.collect::<mongodb::error::Result<Vec<bson::Document>>>()
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

#[get("/authed")]
async fn authed(tok: auth::Token) -> impl Responder {
	format!("Hello, {:?}", tok.user)
}
