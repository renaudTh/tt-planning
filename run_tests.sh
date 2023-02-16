#bin/bash
sqlite3 db.sqlite3 < data.sql
sqlite3 db.sqlite3 < test_data.sql
cd server
cargo test
cd ..
