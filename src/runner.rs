use super::types::Commands;

pub fn run(command: super::types::Command) -> () {
  let track: String = String::from("track");

  match command.name {
    track => super::commands::download_track(Commands::Track(command.args.first().unwrap().to_string()))
  }
}