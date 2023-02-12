
enum Status{
    Office,
    Remote,
    Off,
    Course
}
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}
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
        println!("{:?}", row);  
    }
}
