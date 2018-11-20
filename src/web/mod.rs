mod api;
mod echain;
mod files;
mod html;
mod response;

#[database("sqlite_bridge")]
struct BridgeDbConn(diesel::SqliteConnection);

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", html::routes())
        .mount("/api", api::routes())
        .mount("/static", files::routes())
        .attach(BridgeDbConn::fairing())
}
