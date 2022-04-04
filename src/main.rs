use std::io::{stdin, Read};

use clap::{Parser, Subcommand};
use ::memos::PostgresMemoStore;
use memos::MemoStore;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    method: Methods,
}

#[derive(Subcommand)]
enum Methods {
    Add,
    Find,
}

fn add_memo(store: PostgresMemoStore) {
    let mut name = String::new();
    let mut url = String::new();
    let mut body = String::new();

    println!("Enter name for new memo");
    stdin().read_line(&mut name).unwrap();
    let name = &name[..name.len() - 1];

    println!("Enter url for new memo");
    stdin().read_line(&mut url).unwrap();
    let url = &url[..url.len() - 1];

    println!("Enter body for memo, press {} to finish", EOF);
    stdin().read_to_string(&mut body).unwrap();

    store.write(name, url, body.as_str());
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

fn find_memo(store: PostgresMemoStore) {
    println!("Searching");

    store.find_all();
}

fn main() {
    let store = PostgresMemoStore::new();

    let args = Cli::parse();

    match args.method {
        Methods::Add => add_memo(store),
        Methods::Find => find_memo(store),
    }
}
