use super::types::{Command, DownloadTrack};

pub fn run(command: Command) -> () {
  let track: &str = "track";

  if (track == &command.name) {
    let structure: DownloadTrack = DownloadTrack { url: command.args.first().unwrap().to_string() };
    super::commands::download_track(structure)
  }

//  match command.name.as_str() {
//    track => println!("TRACK?"),
//      let structure: DownloadTrack = DownloadTrack { url: command.args.first().unwrap().to_string() };
//      super::commands::download_track(structure)
//    }
//    _ => println!("Incorrect command")
//  }
}