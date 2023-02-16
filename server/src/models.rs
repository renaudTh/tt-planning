
#[derive(Debug)]
pub enum Status {
    Office,
    Remote,
    Off,
    Course,
}
#[derive(Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

pub fn get_day(v: i64) -> Option<Day> {
    match v {
        0 => Some(Day::Monday),
        1 => Some(Day::Tuesday),
        2 => Some(Day::Wednesday),
        3 => Some(Day::Thursday),
        4 => Some(Day::Friday),
        _ => None,
    }
}
pub fn get_status(v: i64) -> Option<Status> {
    match v {
        0 => Some(Status::Office),
        1 => Some(Status::Remote),
        2 => Some(Status::Off),
        3 => Some(Status::Course),
        _ => None,
    }
}
#[derive(Debug)]
pub struct Presence(pub Day, pub Status, pub Status);

#[derive(Debug)]
pub struct Poll {
    pub year: i64,
    pub week: i64,
    pub votes: Vec<Vote>,
}
#[derive(Debug)]
pub struct Vote {
    pub name: String,
    pub day : Day,
    pub am : Status,
    pub pm : Status,
}
