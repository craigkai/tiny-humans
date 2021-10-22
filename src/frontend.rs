use std::collections::HashMap;
use rocket_dyn_templates::Template;

#[get("/")]
pub async fn index() -> Template {
  let context = HashMap::<String, String>::new();

  Template::render("index", context)
}
