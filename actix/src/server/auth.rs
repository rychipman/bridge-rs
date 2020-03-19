use crate::{
	db::{models::user::User, mongo},
	result::{Error, Result},
};
use actix_web::{dev, http::header, FromRequest, HttpRequest};
use futures::future::Future;
use std::pin::Pin;

pub struct Token {
	pub user: User,
}

impl FromRequest for Token {
	type Config = ();
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output = Result<Token>>>>;

	fn from_request(req: &HttpRequest, pl: &mut dev::Payload) -> Self::Future {
		/*
		let header_val = match req.headers().get(header::AUTHORIZATION) {
			Some(val) => Ok(val),
			None => Err(Error::AuthorizationHeaderMissing),
		}?;
		*/

		let header_val = req
			.headers()
			.get(header::AUTHORIZATION)
			.expect("no authz header");
		let tok = header_val.to_str().expect("header to string")[7..].to_string();

		let fut = mongo::Client::from_request(req, pl);
		Box::pin(async move {
			let mc = fut.await?;
			let user = User::get_by_token(mc, tok)?;
			Ok(Token { user })
		})
	}
}
