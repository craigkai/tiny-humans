#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::tokio::sync::broadcast::channel;
use rocket_dyn_templates::Template;

pub mod database;
pub mod frontend;
pub mod human;

#[rocket::main]
async fn main() {
    database::create();

    if let Err(e) = rocket::build()
        .manage(channel::<human::Message>(1024).0)
        .attach(human::stage())
        .attach(Template::fairing())
        .mount("/", routes![frontend::index, human::events])
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await
    {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    }
}
