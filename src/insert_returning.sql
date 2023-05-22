INSERT INTO test_post (
"id"
, "title"
, "user_id"
) VALUES (
DEfAULT
, $1
, $2)
RETURNING "id"
, "title"
, "user_id"