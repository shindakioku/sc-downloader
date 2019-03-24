use super::types::{Command, DownloadTrack};

pub fn run(command: Command) {
  let track: &str = "track";

  if track == &command.name {
    let structure: DownloadTrack = DownloadTrack { url: command.args.first().unwrap().to_string() };
    return super::commands::download_track(structure);
  }

  println!("Incorrect command");
}