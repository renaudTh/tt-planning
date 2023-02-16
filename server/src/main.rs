
mod models;
mod data_access;

use models::*;
use data_access::*;

fn main() {

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
    println!("{:?}", v);
}
