#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::{NewMemo, Memo};
use schema::memos;
use std::env;

pub mod schema;
pub mod models;


pub trait MemoStore {
    fn write(&self, name: &str, memo_body: &str);
    fn find(&self);
}

pub struct PostgresMemoStore {
    conn: PgConnection,
}

impl MemoStore for PostgresMemoStore{
    fn write<'a>(&self, name: &'a str, body: &'a str) {

//         let new_memo = NewMemo {
//             name,
//             body,
//         };

//         diesel::insert_into(schema::memos::table)
//             .values(&new_memo)
//             .get_result(&self.conn)
//             .expect("Error saving memo")
    }

    fn find(&self) {
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
