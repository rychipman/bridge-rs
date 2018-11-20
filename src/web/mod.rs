mod api;
mod echain;
mod response;

#[database("sqlite_bridge")]
struct BridgeDbConn(diesel::SqliteConnection);

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", api::routes())
        .attach(BridgeDbConn::fairing())
}
