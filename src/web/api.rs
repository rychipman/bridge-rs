use super::response::Message;
use rocket::Route;

#[get("/hello_world")]
fn hello_world() -> Message {
    Message::new("Hello, world!")
}

pub fn routes() -> Vec<Route> {
    routes![hello_world,]
}
