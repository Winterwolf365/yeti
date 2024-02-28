use yeti::controllers::*;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(posts::stage())
}
