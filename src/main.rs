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
    transportation_go: Transportation,
    transportation_back: Transportation,
    hotel: Hotel
}

impl Event {
    // Event構造体をjsonに変換
    fn to_json(&self) -> Json<Event> {
        Json(Event {
            id: self.id,
            name: self.name.to_string(),
            place: self.place.to_string(),
            time: self.time.to_string(),
            transportation_go: Transportation {
                tr_type: self.transportation_go.tr_type.to_string(),
                begin_place: self.transportation_go.begin_place.to_string(),
                end_place: self.transportation_go.end_place.to_string(),
                time: self.transportation_go.time.to_string(),
            },
            transportation_back: Transportation {
                tr_type: self.transportation_back.tr_type.to_string(),
                begin_place: self.transportation_back.begin_place.to_string(),
                end_place: self.transportation_back.end_place.to_string(),
                time: self.transportation_back.time.to_string(),
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

//db以下のファイル一覧を取得する．
// 引数　: 無し
// 返り値: String型の配列
fn get_files() -> Vec<String> {
    // db以下のファイル名を取得
    let paths = fs::read_dir("db/").unwrap();
    let mut files = Vec::new();
    for path in paths {
        files.push(path.unwrap().path().display().to_string());
    }
    files
}

//db以下にファイルを作成(data1, data2...)しており，最終番号を取得する．
// 引数　: 無し
// 返り値: i32型
fn get_tail_id() -> i32 {
    // db以下のファイル名を取得
    let paths = fs::read_dir("db/").unwrap();
    let mut id_list  = Vec::new();
    for path in paths {
        id_list.push(path.unwrap().path().display().to_string());
    }
    let filename = id_list.last().unwrap();
    let id: i32 = filename.chars().nth(7).unwrap() as i32 - '0' as i32;
    id
}

#[derive(Serialize)]
struct Events {
    event_list: Vec<Event>
}

#[get("/list")]
fn get_list() -> Template {
    // jsonを読み込んでイベント一覧の表示
    let files = get_files();
    let mut event_list = Vec::new();
    for file in files {
        event_list.push(read_db(&file));
    }
    let events: Events = Events {
        event_list: event_list
    };

    Template::render("list", &events)
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
    tr_type_go: String,
    begin_place_go: String,
    end_place_go: String,
    time_go: String,
    tr_type_back: String,
    begin_place_back: String,
    end_place_back: String,
    time_back: String,
    hotel_place: String,
    checkin: String
}

#[post("/store", data = "<form_event>")]
fn store(form_event: Form<FormEvent>) -> Flash<Redirect> {
    let event = form_event.into_inner();
    // json形式で保存
    let id: i32 = get_tail_id() + 1;
    let write_json = json!({
    "id": id,
    "name": event.name,
    "place": event.place,
    "time": event.time,
    "transportation_go": {
        "tr_type": event.tr_type_go,
        "begin_place": event.begin_place_go,
        "end_place": event.end_place_go,
        "time": event.time_go
    },
    "transportation_back": {
        "tr_type": event.tr_type_back,
        "begin_place": event.begin_place_back,
        "end_place": event.end_place_back,
        "time": event.time_back
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

#[get("/index")]
fn index() -> Template {
    let context = TemplateContext {
        name: "a".to_string(),
    };
    Template::render("index", &context)
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
