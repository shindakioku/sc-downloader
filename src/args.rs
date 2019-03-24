use std::env;

use super::types;

pub fn skip_file(args: env::Args) -> Option<types::UserInput> {
  let args: types::UserInput = args.collect();

  match args.len() {
    0 => None,
    _ => Some(args.iter().skip(1).rev().cloned().collect()),
  }
}

pub fn to_command(args: types::UserInput) -> types::Command {
  types::Command {
    name: args.first().unwrap().to_string(),
    args: Vec::new(),
  }
}
