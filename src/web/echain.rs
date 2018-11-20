use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{Responder, Response},
};
use std::io::Cursor;

error_chain!{
    foreign_links {
        Diesel(::diesel::result::Error);
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> ::std::result::Result<Response<'r>, Status> {
        // Render the whole error chain to a single string
        let mut rslt = String::new();
        rslt += &format!("Error: {}", self);
        self.iter()
            .skip(1)
            .map(|ce| rslt += &format!(", caused by: {}", ce))
            .for_each(drop);

        // Create JSON response
        let resp = json!({
                "status": "failure",
                "message": rslt,
            })
        .to_string();

        // Respond. The `Ok` here is a bit of a misnomer. It means we
        // successfully created an error response
        Ok(Response::build()
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(resp))
            .finalize())
    }
}
