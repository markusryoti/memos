use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    method: Methods,
}

#[derive(Subcommand)]
enum Methods {
   Add(MemoAddArgs),
   Find(MemoFindArgs),
}

#[derive(Args)]
struct MemoAddArgs {
    name: String,
    url: String,
}

#[derive(Args)]
struct MemoFindArgs {
    name: String,
}

fn add_new_memo(new_memo: MemoAddArgs) {
    println!("{:?}, {:?}", new_memo.name, new_memo.url);
}

fn find_memo(memo: MemoFindArgs) {
    println!("{:?}", memo.name);
}

fn main() {
    let args = Cli::parse();

    match args.method {
        Methods::Add(add_opts) => add_new_memo(add_opts),
        Methods::Find(find_opts) => find_memo(find_opts),
    }
}
