use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel;

use crate::models::{Post, NewPost};

pub mod models;
pub mod schema;

fn main() {
    let connection = &mut connect_db();

    create_post(connection, "First Post", "Hello! This is the first ever blog post!");

    println!("Done, check DB");
}


pub fn connect_db() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL Not specified.");
    
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(connection: &mut SqliteConnection, title: &str, body: &str) {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values((
                    &new_post,
                    posts::date.eq(diesel::dsl::now), // <- HOW ARE THESE TWO AUTROMATICALLY
                    posts::published.eq(false)        // <- RESOLVED IN THE DEMO???
                ))
        .returning(Post::as_returning())
        .get_result(connection)
        .expect("Error saving new post");
}

pub fn list_posts(connection: &mut SqliteConnection) {

    use self::schema::posts::dsl::*;
    let results = posts
        .select(models::Post::as_returning())
        .load(connection)
        .expect("Error Loading Posts");

    for post in results {
        println!("{}", post.title);
        println!("{}", post.body);
        print!("======");
    }
}