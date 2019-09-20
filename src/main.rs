#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;
use rocket::Request;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;

#[derive(Debug, Serialize, Deserialize)]
struct Transportation {
    tr_type: String,
    begin_place: String,
    end_place: String,
    time: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Hotel {
    place: String,
    time: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: i32,
    name: String,
    place: String,
    time: String,
    transportation: Transportation,
    hotel: Hotel
}

impl Event {
    fn to_json(&self) -> Json<Event> {
        Json(Event {
            id: self.id,
            name: self.name.to_string(),
            place: self.place.to_string(),
            time: self.time.to_string(),
            transportation: Transportation {
                tr_type: self.transportation.tr_type.to_string(),
                begin_place: self.transportation.begin_place.to_string(),
                end_place: self.transportation.end_place.to_string(),
                time: self.transportation.time.to_string(),
            },
            hotel: Hotel {
                place: self.hotel.place.to_string(),
                time: self.hotel.time.to_string(),
            }
        })
    }
}

// jsonファイルからデータを読み込んで構造体に変換して返す
// 引数　: &str型(ファイル名)
// 返り値: Event型
fn read_db(filename: &str) -> Event {
    let mut file = File::open(filename).expect("file not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    serde_json::from_str(&data).unwrap()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
fn get_list() -> Template {
    // jsonを読み込んでイベント一覧の表示
    let event: Event = read_db("db/sample.json");

    Template::render("list", &event)
}

#[get("/registration")]
fn registration() -> Template {
    let context = TemplateContext {
        name: "Test".to_string(),
        items: vec!["1", "2"]
    };
    Template::render("form", &context)
}

#[get("/registration_complete")]
fn registration_complete() -> Template {
    let context = TemplateContext {
        name: "Test".to_string(),
        items: vec!["1", "2"]
    };
    Template::render("registration_complete", &context)
}



#[derive(FromForm)]
struct FormEvent {
    name: String,
    place: String,
    time: String,
    tr_type: String,
    begin_place: String,
    end_place: String,
    begin_time: String,
    hotel_place: String,
    checkin: String
}

#[post("/store", data = "<form_event>")]
/*fn store(form_event: Form<Form_event>) {
}
*/
fn store(form_event: Form<FormEvent>) -> Flash<Redirect> {
    let event = form_event.into_inner();
    println!("{}", event.name);
    // 例
    // ----------ここから---------------
    if event.name.is_empty() {
        Flash::error(Redirect::to("/registration_complete"), "Description cannot be empty.")
    } else {
        Flash::error(Redirect::to("/registration_complete"), "Whoops! The server failed.")
    }
    // ----------ここまで---------------
}
// サンプルコード
// ----------ここから---------------
#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>
}

#[get("/hello/<name>")]
fn get(name: String) -> Template {
    // DBから呼び出すようにする
    let list = vec!["test1", "result1"];

    let context = TemplateContext {
        name: name,
        items: list
    };
    Template::render("index", &context)
}

#[get("/input")]
fn input_form() -> Template {
    // DBから呼び出すようにする
    let list = vec!["test1", "result1"];

    let context = TemplateContext {
        name: "a".to_string(),
        items: list
    };
    Template::render("form", &context)
}
// ----------ここまで---------------

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, get_list, registration, store, registration_complete])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
