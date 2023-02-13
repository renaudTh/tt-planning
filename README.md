
## Build instructions

First recreate the database from `data.sql` with
```bash
sqlite3 db.sqlite3 < data.sql
```

Then you can run the server with

```bash
cd server
cargo run
```