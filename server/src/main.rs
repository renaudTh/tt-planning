 #[derive(Debug)]
enum Status{
    Office,
    Remote,
    Off,
    Course
}
 #[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}

fn get_day(v: i64) -> Option<Day> {
    match v {
        0 => Some(Day::Monday),
        1 => Some(Day::Tuesday),
        2 => Some(Day::Wednesday),
        3 => Some(Day::Thursday),
        4 => Some(Day::Friday),
        _ => None
    }
}
fn get_status(v: i64) -> Option<Status> {
    match v {
        0 => Some(Status::Office),
        1 => Some(Status::Remote),
        2 => Some(Status::Off),
        3 => Some(Status::Course),
        _ => None
    }
}
 #[derive(Debug)]
struct Presence(Day, Status, Status);

struct Poll{
    year: u32,
    week: u8
}
struct Vote {
    name : String,
    poll : Poll,
    presence : Vec<Presence>
}
fn main() {
    let connection = sqlite::open("../db.sqlite3").unwrap();

    let query = "SELECT * FROM Vote WHERE poll_id = ?";

    for row in connection
        .prepare(query)
        .unwrap()
        .into_iter()
        .bind((1, 1))
        .unwrap()
        .map(|row| row.unwrap())
    {
        //println!("{:?}", row);  
        let day = get_day(row.read::<i64, _>("day")).unwrap();
        let am = get_status(row.read::<i64, _>("am")).unwrap();
        let pm = get_status(row.read::<i64, _>("pm")).unwrap();
        let presence = Presence(day,am,pm);
        println!("{:?}",presence);  
        
        // println!("{:?}", day);  

        
    }
}
