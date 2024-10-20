-- This file should undo anything in `up.sql`
-- This file should undo anything in `up.sql`
ALTER TABLE "jobs" ALTER COLUMN "id" DROP DEFAULT;
DROP SEQUENCE jobs_id_seq;

DROP TABLE IF EXISTS "jobs";
