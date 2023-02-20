
mod models;
mod data_access;

#[macro_use] extern crate rocket;

#[get("/poll/<id>")]
fn fetch_poll(id:i64) -> String {
    
    let dao = data_access::Dao::new("../db.sqlite3");
    let v = dao.get_poll(id).unwrap();
    format!("{:?}",v)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch_poll])
}
