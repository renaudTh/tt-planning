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
