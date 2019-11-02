#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::path::PathBuf;
use std::path::Path;

use rocket_contrib::json::{Json};
use rocket_contrib::templates::Template;
use rocket::response::NamedFile;

#[macro_use]
extern crate dotenv_codegen;
extern crate slack_hook;
use slack_hook::{Slack, PayloadBuilder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Message {
    email: String
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "ok");
    return Template::render("index", &context);
}

#[get("/js/<path..>")]
fn js(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/js").join(path)).ok()
}

#[get("/css/<path..>")]
fn css(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/css").join(path)).ok()
}

#[get("/images/<path..>")]
fn images(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/images").join(path)).ok()
}

#[get("/robots.txt")]
fn robots() -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/robots.txt")).ok()
}

#[post("/submit", data = "<message>")]
fn submit(message: Json<Message>) -> String {
    let slack = Slack::new(dotenv!("SLACK_WEBHOOK")).unwrap();
    let p = PayloadBuilder::new()
      .text(format!("email: {}", message.0.email))
      .channel(format!("#{}", dotenv!("SLACK_CHANNEL_NAME")))
      .username(dotenv!("SLACK_USER_NAME"))
      .icon_emoji(":chart_with_upwards_trend:")
      .build()
      .unwrap();

    let res = slack.send(&p);
    if res.is_ok() {
        return "ok".to_string();
    } else {
        return "ng".to_string();
    }
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, js, css, images, submit, robots])
    .attach(Template::fairing())
    .launch();
}