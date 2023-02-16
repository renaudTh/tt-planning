
use rusqlite::{Connection, Result};
use super::models::*;

pub struct Dao{
    connection: Connection
}
impl Dao{
    pub fn new(path:&str) -> Dao{
        
        Dao{
            connection: Connection::open(path).unwrap()
        }
    }

    fn get_distinct_names(&self, poll_id:i64) -> Result<Vec<String>>{
        let mut ret : Vec<String> = Vec::new();
        let mut statement = self.connection.prepare("SELECT DISTINCT(name) FROM Vote WHERE poll_id = ?")?;
        let mut result_iter = statement.query([poll_id])?;

        while let Some(row) = result_iter.next()? {
            let value = row.get(0).unwrap();
            ret.push(value);
        }
        Ok(ret)
       
    }   
   
}
#[cfg(test)]    
mod tests {
    use super::*;

    #[test]
    fn get_distinct_names_test() {
        
        let dao = Dao::new("../db.sqlite3");
        let res = dao.get_distinct_names(1).unwrap();
        println!("{:?}", res);
        assert!(res.contains(&String::from("Simon")));
        assert!(res.contains(&String::from("Thomas")));
        assert_eq!(res.len(),3);
    }
}