use sqlx::sqlite::SqlitePoolOptions;
use dotenvy::dotenv;
use std::env;

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct DbRow { id: i32 , title: String , description: Option<String> }

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not specified");

    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await?;

    sqlx::query("INSERT INTO wyrite (title, description) VALUES('first-entry', 'This is the first database entry')")
        .execute(&pool).await?;

    let stream = sqlx::query_as::<_, DbRow>("SELECT * FROM wyrite")
        .fetch_one(&pool).await?;

    println!("{:?}", stream);

    Ok(())
}
