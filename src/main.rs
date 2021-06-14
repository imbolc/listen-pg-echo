use anyhow::Result;
use clap::Clap;
use sqlx::{Connection, PgConnection};
use std::time::{Duration, Instant};

#[derive(Clap)]
struct Opts {
    #[clap(long, default_value = "postgres:///listen_postgres_echo")]
    db_url: String,
    #[clap(long, default_value = "10000")]
    limit: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let mut con = PgConnection::connect(&opts.db_url).await?;
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        sqlx::query("NOTIFY echo").execute(&mut con).await.unwrap();
    });

    let mut cnt = 0;
    let mut started_at = None;
    let mut last_notification = None;
    let mut con = PgConnection::connect(&opts.db_url).await?;
    let mut listener = sqlx::postgres::PgListener::connect(&opts.db_url).await?;
    listener.listen("echo").await?;
    while cnt < opts.limit {
        last_notification = Some(listener.recv().await?);
        if cnt == 0 {
            started_at = Some(Instant::now());
        }
        cnt += 1;
        sqlx::query(&format!("NOTIFY echo, '{}'", cnt))
            .execute(&mut con)
            .await
            .unwrap();
    }

    let took = Instant::now() - started_at.unwrap();
    let secs = took.as_secs() as f64 + took.subsec_nanos() as f64 * 1e-9;
    let per_second = cnt as f64 / secs;
    dbg!(last_notification);
    dbg!(took);
    dbg!(per_second);

    Ok(())
}
