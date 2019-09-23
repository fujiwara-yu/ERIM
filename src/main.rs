#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::fs;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;
use rocket::Request;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use serde_json::json;

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

fn get_tail_id() -> i32 {
    // db以下のファイル名を取得
    let paths = fs::read_dir("db/").unwrap();
    let mut id: String;
    for path in paths {
        id = path.unwrap().path().display().to_string();
    }
//    let tail = id.chars().nth(7).unwrap() as i32 - 48;
    1
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
fn get_list() -> Template {
    // jsonを読み込んでイベント一覧の表示
    let id = get_tail_id();
    let filename = "db/data".to_string() + &id.to_string() + ".json";
    let event: Event = read_db(&filename);

    Template::render("list", &event)
}

#[get("/registration")]
fn registration() -> Template {
    let context = TemplateContext {
        name: "Test".to_string(),
    };
    Template::render("form", &context)
}

#[get("/registration_complete")]
fn registration_complete() -> Template {
    let context = TemplateContext {
        name: "Test".to_string(),
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
fn store(form_event: Form<FormEvent>) -> Flash<Redirect> {
    let event = form_event.into_inner();
    // json形式で保存
    let id: i32 = get_tail_id();
    let write_json = json!({
    "id": id,
    "name": event.name,
    "place": event.place,
    "time": event.time,
    "transportation": {
        "tr_type": event.tr_type,
        "begin_place": event.begin_place,
        "end_place": event.end_place,
        "time": event.begin_time
    },
    "hotel": {
        "place": event.hotel_place,
        "time": event.checkin
    }
});
    let filename = "db/data".to_string() + &id.to_string() + ".json";
    let mut f = File::create(filename).unwrap();
    writeln!(f, "{}", write_json);

    Flash::success(Redirect::to("/registration_complete"), "OK")
}
// サンプルコード
// ----------ここから---------------
#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/hello/<name>")]
fn get(name: String) -> Template {
    let context = TemplateContext {
        name: name,
    };
    Template::render("index", &context)
}

#[get("/input")]
fn input_form() -> Template {
    let context = TemplateContext {
        name: "a".to_string(),
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
