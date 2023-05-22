create table if not exists test_post(
id serial primary key
, title text
, user_id integer
-- DEFERRABLE INITIALLY DEFERRED is what causes the error
, constraint fk_user foreign key(user_id) references test_user(id) DEFERRABLE INITIALLY DEFERRED
);