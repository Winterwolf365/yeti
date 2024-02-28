use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::schema::posts::dsl::*;
use crate::{models::post::Post, services::database};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Posts Stage", |rocket| async move {
        rocket.mount("/posts", routes![retrive_all, retrive, delete])
    })
}

#[get("/")]
fn retrive_all() -> Result<Json<Vec<Post>>, Status> {
    let db = &mut database::establish_connection();

    match posts.load::<Post>(db) {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<post_id>")]
fn retrive(post_id: i32) -> Result<Json<Post>, Status> {
    let db = &mut database::establish_connection();

    match posts.find(post_id).first::<Post>(db).optional() {
        Ok(Some(res)) => Ok(Json(res)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<post_id>")]
fn delete(post_id: i32) -> Status {
    let db = &mut database::establish_connection();

    if let Ok(None) = posts.find(post_id).first::<Post>(db).optional() {
        return Status::NotFound;
    }

    match diesel::delete(posts.find(post_id)).execute(db) {
        Ok(_) => Status::NoContent,
        Err(_) => Status::InternalServerError,
    }
}
