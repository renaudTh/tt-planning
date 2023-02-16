
mod models;
mod data_access;

use models::*;
use data_access::*;
fn main() {
    let dao = Dao::new("../db.sqlite3");

    let v = Vote{
        name: String::from("Clémence"),
        presence: vec![
            Presence(Day::Monday, Status::Office, Status::Office),
            Presence(Day::Tuesday, Status::Office, Status::Office),
            Presence(Day::Wednesday, Status::Remote, Status::Remote),
            Presence(Day::Thursday, Status::Office, Status::Office),
            Presence(Day::Friday, Status::Remote, Status::Remote)
        ]
    };
    
    let insert = dao.add_vote(1, v);
    match insert {
        Ok(_) => println!("Vote added !"),
        Err(err) => println!("{}", err)
    }
}
