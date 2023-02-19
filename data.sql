BEGIN TRANSACTION;
DROP TABLE IF EXISTS "Poll";
CREATE TABLE IF NOT EXISTS "Poll" (
	"id"	INTEGER NOT NULL,
	"year"	INTEGER NOT NULL,
	"week"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
DROP TABLE IF EXISTS "Vote";
CREATE TABLE IF NOT EXISTS "Vote" (
	"poll_id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"day"	INTEGER NOT NULL,
	"am"	INTEGER DEFAULT 0,
	"pm"	INTEGER DEFAULT 0,
	FOREIGN KEY("poll_id") REFERENCES "Poll"("id"),
	PRIMARY KEY("poll_id","name","day")
);
COMMIT;


/*  
Poll(poll_id, year, week)
Vote(poll_id, person, day, am, pm) 

Vote {
	poll_id : 1
	person : "Thomas",
	presences : [
		{
			day: "Monday",
			am: "Remote",
			pm: "Office"
		}
		{
			day:"Tuesday",
			am: "Office",
			pm: "Office"
		}
	]
}

*/