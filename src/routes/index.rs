use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    let context = std::collections::HashMap::<String, String>::new();
    Template::render("index", &context)
}