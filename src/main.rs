use std::env;

mod args;
mod commands;
mod types;

fn main() {
    match args::skip_file(env::args()) {
        None => println!("Woops"),
        Some(args) => commands::run(args::to_hash(args)),
    }
}
