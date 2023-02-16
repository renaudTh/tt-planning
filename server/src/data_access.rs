use sqlite::{Connection, Error};
use super::models::*;

pub struct Dao {
    connection: Connection,
}
impl Dao {
    pub fn new(path: &str) -> Dao {
        Dao {
            connection: sqlite::open(path).unwrap(),
        }
    }

    pub fn get_distinct_names(&self, id: i64) -> Vec<String> {
        let query = "SELECT DISTINCT(name) FROM Vote WHERE poll_id = ?";
        let ret = self
            .connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .bind((1, id))
            .unwrap()
            .map(|r| r.unwrap())
            .map(|r| String::from(r.read::<&str, _>("name")))
            .collect();
        return ret;
    }
    pub fn get_presences(&self, poll_id: i64, name: &str) -> Vec<Presence> {
        let query = "SELECT day,am,pm FROM Vote WHERE poll_id = ? AND name = ?";
        let ret = self
            .connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .bind((1, poll_id))
            .unwrap()
            .bind((2, name))
            .unwrap()
            .map(|r| r.unwrap())
            .map(|r| {
                Presence(
                    get_day(r.read::<i64, _>("day")).unwrap(),
                    get_status(r.read::<i64, _>("am")).unwrap(),
                    get_status(r.read::<i64, _>("pm")).unwrap(),
                )
            })
            .collect();
        return ret;
    }
    pub fn get_all_votes(&self, poll_id: i64) -> Vec<Vote> {
        let mut ret: Vec<Vote> = Vec::new();
        let names = self.get_distinct_names(poll_id);
        for name in names {
            let presence = self.get_presences(poll_id, &name);
            ret.push(Vote { name, presence });
        }
        ret
    }
    pub fn get_poll(&self, year: i64, week: i64) -> Poll {
        let query = "SELECT * FROM Poll WHERE year = ? AND week = ?";
        let ret = self
            .connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .bind((1, year))
            .unwrap()
            .bind((2, week))
            .unwrap()
            .map(|r| r.unwrap())
            .map(|r| Poll {
                year: r.read::<i64, _>("year"),
                week: r.read::<i64, _>("week"),
                votes: self.get_all_votes(r.read::<i64, _>("id")),
            })
            .last()
            .unwrap();

        return ret;
    }

    pub fn add_presence(&self, poll_id: i64, name: &str, presence: Presence) -> Result<(), Error> {
        let query = "INSERT INTO Vote VALUES (?,?,?,?,?)";
        let mut statement = self.connection.prepare(query)?;

        statement.bind((1,poll_id))?;
        statement.bind((2,name))?;
        statement.bind((3,presence.0 as i64))?;
        statement.bind((4,presence.1 as i64))?;
        statement.bind((5,presence.2 as i64))?;
        statement.next()?;
        Ok(())
    }
    //TODO : too slow :( Change for a transaction instead of multiple inserts inside a loop. 
    pub fn add_vote(&self, poll_id : i64, vote: Vote) -> Result<(), Error>{
        for presence in vote.presence {
            self.add_presence(poll_id, &vote.name, presence)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_distinct_names_test() {
        let dao = Dao::new("../db.sqlite3");
        let names = dao.get_distinct_names(1);
        assert_eq!(names.len(),2);
        assert!(names.contains(&String::from("Thomas")));
        assert!(names.contains(&String::from("Simon")));
    }
}