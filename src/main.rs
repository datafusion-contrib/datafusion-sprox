#[macro_use]
extern crate rocket;
mod config;
mod models;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::index, routes::status::health])
}
