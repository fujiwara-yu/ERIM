#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;


use std::collections::HashMap;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>
}

#[get("/hello/<name>")]
fn get(name: String) -> Template {
    // DBから呼び出す
    let list = vec!["test1", "result1"];

    let context = TemplateContext {
        name: name,
        items: list
    };
    Template::render("index", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
