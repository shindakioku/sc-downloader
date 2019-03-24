use std::env;

mod args;
mod runner;
mod types;
mod commands;

fn main() {
    match args::skip_file(env::args()) {
        None => println!("Woops"),
        Some(args) => runner::run(args::to_command(args)),
    }
}
