use std::fs;
#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct Human {
  position: Vec<i32>,
  pose: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Humans {
    humans: Vec<Human>
}

#[get("/")]
fn index() -> Template {
    let context = fs::read_to_string("database.json").expect("Unable to read file");
    let humans: Humans = serde_json::from_str( &context ).unwrap();

    Template::render("index", humans)
}



#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await {

        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    }
}
