use std::{env, time::Duration};

use dotenvy::dotenv;
use sqlx::{query, Connection, SqliteConnection};
use tokio;

use super::logic_test::*;

#[tokio::test]
async fn create_entry() {
    dotenv().unwrap();

    let database_url = env::var("DATABASE_URL").unwrap();
    let mut conn = SqliteConnection::connect(&database_url).await.unwrap();

    save_result(5, "__testname__", Duration::from_secs(5))
        .await
        .unwrap();

    query!("DELETE FROM Result WHERE Result.name = \"__testname__\"")
        .execute(&mut conn)
        .await
        .unwrap();
}

#[tokio::test]
async fn read_database() {
    get_leaderboard().await.unwrap();
}
