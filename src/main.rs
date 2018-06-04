#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;
use rocket_contrib::JSON;

#[derive(Serialize)]
struct SerializedResult {
    result: String,
}

#[get("/")]
fn index() -> &'static str {
    "Braindead Rust Web Calculator!"
}

#[get("/add/template/<a>/<b>")]
fn addTemplate(a: i32, b: i32) -> Template {
    let context = SerializedResult {
        result: (a+b).to_string()
    };

    Template::render("index", &context)
}

#[get("/add/json/<a>/<b>")]
fn addJson(a: i32, b: i32) -> Json<SerializedResult> {
    Json(SerializedResult { result: (a+b).to_string() })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, addTemplate, addJson])
        .attach(Template::fairing())
        .launch();
}