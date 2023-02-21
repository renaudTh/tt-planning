
mod models;
mod data_access;

#[macro_use] extern crate rocket;
use rocket::serde::json::{Json,Value,json};


#[get("/poll/<id>")]
fn fetch_poll(id:i64) -> Json<Vec<models::Vote>> {
    
    let dao = data_access::Dao::new("../db.sqlite3");
    let v = dao.get_poll(id).unwrap();
    Json(v)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch_poll])
}
