use rocket::serde::json::{json, Value};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

fn colors() -> HashMap<&'static str, &'static str> {
    let mut colors = HashMap::new();
    colors.insert("PersonRed", "invert(12%) sepia(100%) saturate(4004%) hue-rotate(241deg) brightness(100%) contrast(150%);");
    colors.insert("PersonBlue", "invert(24%) sepia(96%) saturate(7498%) hue-rotate(359deg) brightness(105%) contrast(109%);");
    colors.insert(
        "PersonGreen",
        "invert(48%) sepia(79%) saturate(2476%) hue-rotate(86deg) brightness(118%) contrast(119%);",
    );
    colors
}

#[get("/")]
pub async fn index() -> Template {
    let mut context = HashMap::<String, Value>::new();

    context.insert("colors".to_string(), json!(colors()));
    Template::render("index", context)
}

#[test]
fn test_colors() {
    assert_eq!(colors().len(), 3);
}
