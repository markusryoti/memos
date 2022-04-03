#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub trait MemoStore {
    fn write(&self);
    fn find(&self);
}

pub struct PostgresMemoStore {
    conn: PgConnection,
}

impl MemoStore for PostgresMemoStore{

    fn write(&self) {
        println!("Writing");
    }

    fn find(&self) {

    }
}

impl PostgresMemoStore {
   pub fn new() -> Self {
       PostgresMemoStore { conn: establish_connection() }
   }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
