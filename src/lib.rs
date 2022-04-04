#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::{Memo, NewMemo};

pub trait MemoStore {
    fn write<'a>(&self, name: &'a str, url: &'a str, memo_body: &'a str);
    fn find_all(&self);
}

pub struct PostgresMemoStore {
    conn: PgConnection,
}

impl MemoStore for PostgresMemoStore {
    fn write<'a>(&self, name: &'a str, url: &'a str, body: &'a str) {
        let new_memo = NewMemo { name, url, body };

        diesel::insert_into(schema::memos::table)
            .values(&new_memo)
            .get_result::<Memo>(&self.conn)
            .expect("Error while creating memo");
    }

    fn find_all(&self) {
        let results = schema::memos::dsl::memos
            .load::<Memo>(&self.conn)
            .expect("Error loading memos");

        println!("Printing all results");
        for memo in results {
            println!("{}", memo.name);
        }
    }
}

impl PostgresMemoStore {
    pub fn new() -> Self {
        PostgresMemoStore {
            conn: establish_connection(),
        }
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
