use super::types::DownloadTrack;

pub fn download_track(track: DownloadTrack) -> () {
  println!("{}", track.url);
}