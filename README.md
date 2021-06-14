Postgres listen / notify echo benchmark
=======================================

Tokio & sqlx

```
cargo run --release -- --limit=10000 --db-url=postgres:///listen_postgres_echo

[src/main.rs:45] last_notification = Some(
    PgNotification {
        process_id: 24266,
        channel: "echo",
        payload: "9999",
    },
)
[src/main.rs:46] took = 1.034358464s
[src/main.rs:47] per_second = 9667.828270412838
```
