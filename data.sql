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
INSERT INTO "Poll" ("id","year","week") VALUES (1,2023,7);
INSERT INTO "Poll" ("id","year","week") VALUES (2,2023,8);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Thomas',0,0,0);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Simon',0,1,1);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Thomas',1,0,0);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Thomas',2,2,2);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Thomas',3,2,2);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Thomas',4,2,2);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Simon',1,1,1);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Simon',2,1,2);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Simon',3,2,2);
INSERT INTO "Vote" ("poll_id","name","day","am","pm") VALUES (1,'Simon',4,2,2);
COMMIT;
