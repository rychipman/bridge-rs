use crate::result::{Error, Result};
use actix_web::{dev, web, FromRequest, HttpRequest};
use futures::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct Client(web::Data<mongodb::Client>);

impl FromRequest for Client {
	type Config = ();
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output = Result<Client>>>>;

	fn from_request(req: &HttpRequest, pl: &mut dev::Payload) -> Self::Future {
		let fut = web::Data::<mongodb::Client>::from_request(req, pl);
		Box::pin(async move {
			let data = fut.await?;
			Ok(Client(data))
		})
	}
}

impl std::ops::Deref for Client {
	type Target = mongodb::Client;

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}
