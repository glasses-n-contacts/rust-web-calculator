#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Template};

#[derive(Deserialize)]
struct Parameters {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
struct SerializedResult {
    result: String,
}

#[get("/")]
fn index() -> &'static str {
    "Braindead Rust Web Calculator!"
}

#[get("/add/template/<a>/<b>")]
fn add_template(a: i32, b: i32) -> Template {
    let context = SerializedResult {
        result: (a+b).to_string()
    };

    Template::render("index", &context)
}

#[get("/add/json/<a>/<b>")]
fn add_json(a: i32, b: i32) -> Json<SerializedResult> {
    Json( SerializedResult { result: (a+b).to_string() })
}

#[post("/add_post", data = "<params>")]
fn add_post(params: Json<Parameters>) -> Json<SerializedResult> {
    Json( SerializedResult { result: (params.a + params.b).to_string() })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, add_template, add_json, add_post])
        .attach(Template::fairing())
        .launch();
}