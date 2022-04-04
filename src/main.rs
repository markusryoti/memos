use std::io::{stdin, Read};

use clap::{Parser, Subcommand};
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

fn add_memo() {
    let mut url = String::new();
    let mut body = String::new();

    println!("Enter URL for new memo");
    stdin().read_line(&mut url).unwrap();
    let url = &url[..url.len() - 1];
    println!("{}", url);

    println!("Enter body for memo, press {} to finish", EOF);
    stdin().read_to_string(&mut body).unwrap();

    println!("{}", body);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

fn find_memo() {
    println!("Searching");
}

fn main() {
    // let args = Cli::parse();

    // match args.method {
    //     Methods::Add => add_memo(),
    //     Methods::Find => find_memo(),
    // }

    let store = memos::PostgresMemoStore::new();

    store.write("first memo", "this is the memo body");
}
