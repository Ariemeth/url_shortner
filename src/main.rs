//#[macro_use]
//extern crate rocket;
use rocket::{get, launch, post, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<input>", format = "text")]
async fn post_new(input: &str) -> String {
    format!("Data: {input}")
}

#[get("/<shortener>")]
async fn get_data(shortener: &str) -> String {
    format!("Shortener: {shortener}")
}

#[get("/monster/<name>")]
async fn monster(name: &str) -> String {
    format!("Hello, monster {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_data, post_new, monster])
}
