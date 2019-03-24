use std::env;

use super::types;

pub fn skip_file(args: env::Args) -> Option<types::UserInput> {
  let args: types::UserInput = args.collect();
  let args: types::UserInput = args.iter().skip(1).cloned().collect();

  match args.len() {
    0 => None,
    _ => Some(args),
  }
}

pub fn to_command(args: types::UserInput) -> types::Command {
  types::Command {
    name: args.first().unwrap().to_string(),
    args: args.iter().skip(1).cloned().collect(),
  }
}
