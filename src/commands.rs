use super::types::DownloadTrack;
use super::sc::*;

pub fn download_track(track: DownloadTrack) {
  download(sound_link(stream_url(page_text(track.url).unwrap())).unwrap())
}