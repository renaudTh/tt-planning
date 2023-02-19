use super::models::*;
use rusqlite::{params, Connection, Result};

pub struct Dao {
    connection: Connection,
}
impl Dao {
    pub fn new(path: &str) -> Dao {
        Dao {
            connection: Connection::open(path).unwrap(),
        }
    }

    fn get_distinct_names(&self, poll_id: i64) -> Result<Vec<String>> {
        let mut ret: Vec<String> = Vec::new();
        let mut statement = self
            .connection
            .prepare("SELECT DISTINCT(name) FROM Vote WHERE poll_id = ?")?;
        let mut result_iter = statement.query([poll_id])?;

        while let Some(row) = result_iter.next()? {
            let value = row.get(0).unwrap();
            ret.push(value);
        }
        Ok(ret)
    }
    fn get_poll(&self, poll_id: i64) -> Result<Vec<Vote>> {
        let mut ret: Vec<Vote> = Vec::new();
        let mut statement = self
            .connection
            .prepare("SELECT * FROM Vote WHERE poll_id = ? ORDER BY name")?;
        let mut result_iter = statement.query([poll_id])?;

        while let Some(row) = result_iter.next()? {
            ret.push(Vote {
                name: row.get("name").unwrap(),
                day: get_day(row.get("day").unwrap()).unwrap(),
                am: get_status(row.get("am").unwrap()).unwrap(),
                pm: get_status(row.get("pm").unwrap()).unwrap(),
            })
        }
        Ok(ret)
    }

    fn insert_vote(&self, poll_id: i64, votes: Vec<Vote>) -> Result<()> {
        for vote in votes {
            self.insert_day_vote(poll_id, vote)?;
        }
        Ok(())
    }
    fn insert_day_vote(&self, poll_id: i64, vote: Vote) -> Result<()> {
        self.connection.execute(
            "INSERT INTO Vote (poll_id, name, day, am, pm) VALUES (?,?,?,?,?)",
            params![
                poll_id,
                vote.name,
                vote.day as i64,
                vote.am as i64,
                vote.pm as i64
            ],
        )?;
        Ok(())
    }

    fn delete_vote(&self, poll_id: i64, name: &str) -> Result<()> {
        self.connection.execute(
            "DELETE FROM Vote WHERE poll_id = ? AND name = ?",
            params![poll_id, name],
        )?;
        Ok(())
    }

    fn update_vote(&self, poll_id: i64, vote: Vote) -> Result<()> {
        self.connection.execute(
            "UPDATE Vote SET am = ?, pm = ? WHERE poll_id = ? AND name = ? and day = ?",
            params![
                vote.am as i64,
                vote.pm as i64,
                poll_id,
                vote.name,
                vote.day as i64
            ],
        )?;
        Ok(())
    }

    fn update_complete_vote(&self, poll_id: i64, votes: Vec<Vote>) -> Result<()> {
        for vote in votes {
            self.update_vote(poll_id, vote)?;
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
        assert!(dao.get_distinct_names(1).is_ok());
    }

    #[test]
    fn get_presences_test() {
        let dao = Dao::new("../db.sqlite3");
        assert!(dao.get_poll(1).is_ok());
    }

    #[test]
    fn insert_presence_test() {
        let dao = Dao::new("../db.sqlite3");
        let v = Vote {
            name: String::from("Clémence"),
            day: Day::Monday,
            am: Status::Office,
            pm: Status::Remote,
        };
        assert!(dao.insert_day_vote(1, v).is_ok());
    }

    #[test]
    fn insert_complete_vote() {
        let name = "Jehan";
        let dao = Dao::new("../db.sqlite3");
        let votes = vec![
            Vote {
                name: String::from(name),
                day: Day::Monday,
                am: Status::Office,
                pm: Status::Office,
            },
            Vote {
                name: String::from(name),
                day: Day::Tuesday,
                am: Status::Office,
                pm: Status::Office,
            },
            Vote {
                name: String::from(name),
                day: Day::Wednesday,
                am: Status::Office,
                pm: Status::Office,
            },
            Vote {
                name: String::from(name),
                day: Day::Thursday,
                am: Status::Office,
                pm: Status::Remote,
            },
            Vote {
                name: String::from(name),
                day: Day::Friday,
                am: Status::Off,
                pm: Status::Off,
            },
        ];
        assert!(dao.insert_vote(1, votes).is_ok());
    }

    #[test]
    fn delete_vote_test() {
        let dao = Dao::new("../db.sqlite3");
        assert!(dao.delete_vote(1, "Thomas").is_ok());
    }
    #[test]
    fn update_vote_test() {
        let dao = Dao::new("../db.sqlite3");
        assert!(dao
            .update_vote(
                1,
                Vote {
                    name: String::from("Simon"),
                    day: Day::Monday,
                    am: Status::Off,
                    pm: Status::Off
                }
            )
            .is_ok());
    }

    #[test]
    fn update_complete_vote_test() {
        let name = "Simon";
        let dao = Dao::new("../db.sqlite3");
        let votes = vec![
            Vote {
                name: String::from(name),
                day: Day::Monday,
                am: Status::Off,
                pm: Status::Off,
            },
            Vote {
                name: String::from(name),
                day: Day::Tuesday,
                am: Status::Off,
                pm: Status::Off,
            },
            Vote {
                name: String::from(name),
                day: Day::Wednesday,
                am: Status::Off,
                pm: Status::Off,
            },
            Vote {
                name: String::from(name),
                day: Day::Thursday,
                am: Status::Off,
                pm: Status::Off,
            },
            Vote {
                name: String::from(name),
                day: Day::Friday,
                am: Status::Off,
                pm: Status::Off,
            },
        ];
        assert!(dao.update_complete_vote(1, votes).is_ok());
    }
}
