Reproduction repo for https://github.com/launchbadge/sqlx/issues/1370

Run with `just run`

The error is caused by improper handling of DEFERRABLE INITIALLY DEFERRED constraints.

That constraint is created in `create_table_posts.sql`. Remove the DEFERRABLE INITIALLY DEFERRED part and the error goes away.

The solution is that fetch_one needs to check whether the connection has been made dirty.

Issue includes a workaround, to explicitly not use `.fetch_one`, since it's broken in this situation, and 
instead use `.fetch`

```rust
async fn main() {
    let mut stream = sqlx::query_as("insert into bar values ('foo') returning *").fetch(&pool);

    let bar: Bar = stream.try_next().await?.ok_or(sqlx::Error::RowNotFound)?;
    // check for deferred FK constraint error
    stream.try_next().await?;
}
```