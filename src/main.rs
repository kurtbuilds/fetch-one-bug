use sqlx::{Connection, Executor, query, Row};
use chrono::{Utc};
use serde_json::Value;
use rust_decimal::Decimal;
use uuid::Uuid;

type DateTime<T = Utc> = chrono::DateTime<T>;

#[tokio::main]
async fn main() {
    let mut conn = sqlx::postgres::PgConnection::connect(
        "postgres://postgres:postgres@localhost:5432/postgres"
    ).await.unwrap();
    conn.execute(include_str!("create_table_user.sql")).await.unwrap();

    conn.execute(include_str!("create_table_posts.sql")).await.unwrap();

    // Let's first demo the correct behavior, on fetch_all
    let mut q = query(include_str!("insert_returning.sql"));
    q = q.bind("title");
    q = q.bind(2); // user_id that doesn't exist

    let rows = q.fetch_all(&mut conn).await;
    assert!(rows.is_err(), "Should error because the insertion violates FK constraint.");

    // Now let's see the wrong behavior on fetch_one. It gives back a row even when the insertion
    // violates a FK constraint.
    let mut q = query(include_str!("insert_returning.sql"));
    q = q.bind("title");
    q = q.bind(Some(2) as Option<i32>); // user_id that doesn't exist

    let row = q.fetch_one(&mut conn).await;
    match row {
        Ok(row) => {
            let id = row.get::<i32, _>("id");
            println!("BUG: fetch_one didn't return an error. fetch_one returned a row with id={}", id);
            println!("The connection is also dirty now. We have to commit, or the next query will fail. Note that COMMIT is an error");
            let res = conn.execute("commit").await;
            println!("commit: is_err={}", res.is_err());
            println!("However, if we query the database, we'll find no results:");
            let results = query("SELECT * FROM test_post").fetch_all(&mut conn).await.unwrap();
            println!("rows in test_post: {}", results.len());
            panic!("BUG: fetch_one didn't return an error");
        }
        Err(_) => {
            println!("Correctly errored on fetch_one");
        }
    }


}