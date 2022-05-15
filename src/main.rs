//#[macro_use]
//extern crate rocket;
use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/monster/<name>")]
async fn monster(name: &str) -> String {
    format!("Hello, monster {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, monster])
}
