use std::env;

mod args;
mod types;

fn main() {
    match args::skip_file(env::args()) {
        None => println!("Woops"),
        Some(args) => handle(args::to_hash(args)),
    }
}

fn handle(command: types::Command) {}
