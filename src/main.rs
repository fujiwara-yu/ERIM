#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;
use rocket::Request;
use rocket::response::Redirect;
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
       //struct to Json
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


//fn read_db(id: i32, event_list: &mut Vec<Event>) {
fn read_db() -> Event {
    // jsonからデータを取得
    let mut file = File::open("db/sample.json").expect("file not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    serde_json::from_str(&data).unwrap()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
fn get_list() {
    // jsonを読み込んでイベント一覧の表示
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
        .mount("/", routes![index, get, input_form])
        .attach(Template::fairing())
}

fn main() {
    read_db();
    rocket().launch();
}
