extern crate core;

mod edit;
mod file;
mod row;

use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();

    edit::Editor::new(args.filename.unwrap_or_else(|| "".into())).run();
}
