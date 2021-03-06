use actix_web::http::header;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	UserNotFound,
	ExerciseNotFound,
	ExerciseBidNotFound,
	DealNotFound,
	UserAlreadyExists,
	IncorrectPassword,
	InvalidSession,
	AuthorizationHeaderMissing,
	MongoError(mongodb::error::Error),
	BsonDecoderError(bson::DecoderError),
	BsonEncoderError(bson::EncoderError),
	BsonObjectIdError(bson::oid::Error),
	ActixHttpError(actix_http::error::Error),
	BcryptError(bcrypt::BcryptError),
	HeaderToStringError(header::ToStrError),
	BridgeError(bridge_core::Error),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl actix_web::error::ResponseError for Error {}

impl From<mongodb::error::Error> for Error {
	fn from(e: mongodb::error::Error) -> Self {
		Error::MongoError(e)
	}
}

impl From<bson::DecoderError> for Error {
	fn from(e: bson::DecoderError) -> Self {
		Error::BsonDecoderError(e)
	}
}

impl From<bson::EncoderError> for Error {
	fn from(e: bson::EncoderError) -> Self {
		Error::BsonEncoderError(e)
	}
}

impl From<bson::oid::Error> for Error {
	fn from(e: bson::oid::Error) -> Self {
		Error::BsonObjectIdError(e)
	}
}

impl From<actix_http::error::Error> for Error {
	fn from(e: actix_http::error::Error) -> Self {
		Error::ActixHttpError(e)
	}
}

impl From<bcrypt::BcryptError> for Error {
	fn from(e: bcrypt::BcryptError) -> Self {
		Error::BcryptError(e)
	}
}

impl From<header::ToStrError> for Error {
	fn from(e: header::ToStrError) -> Self {
		Error::HeaderToStringError(e)
	}
}

impl From<bridge_core::Error> for Error {
	fn from(e: bridge_core::Error) -> Self {
		Error::BridgeError(e)
	}
}
