use sqlite::Connection;

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

struct Dao {
    connection : Connection
}
impl Dao {
    fn new(connection : Connection) -> Dao {
        Dao {
            connection
        }
    }
    
    pub fn get_distinct_names(&self, id:i64) -> Vec<String> {
        
        let query = "SELECT DISTINCT(name) FROM Vote WHERE poll_id = ?";
        let ret = self.connection.prepare(query)
                        .unwrap()
                        .into_iter()
                        .bind((1,id))
                        .unwrap()
                        .map(|r| r.unwrap())
                        .map(|r| String::from(r.read::<&str,_>("name")))
                        .collect();
        return ret;
    }
    pub fn get_presences(&self, poll_id:i64, name: &str) -> Vec<Presence>{
        let query = "SELECT day,am,pm FROM Vote WHERE poll_id = ? AND name = ?";
        let ret = self.connection.prepare(query)
                        .unwrap()
                        .into_iter()
                        .bind((1,poll_id))
                        .unwrap()
                        .bind((2,name))
                        .unwrap()
                        .map(|r| r.unwrap())
                        .map(|r| Presence(get_day(r.read::<i64,_>("day")).unwrap(), 
                                               get_status(r.read::<i64,_>("am")).unwrap(), 
                                               get_status(r.read::<i64,_>("pm")).unwrap()))
                        .collect();
        return ret;
    }

}

fn main() {
    let connection = sqlite::open("../db.sqlite3").unwrap();
    let dao = Dao::new(connection);

    let presences = dao.get_presences(1, "Thomas");

    for pres in presences {
        println!("{:?}", pres);
    }

}
