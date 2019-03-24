extern crate regex;
extern crate reqwest;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;

const CLIENT_ID: &str = "NmW1FlPaiL94ueEu7oziOWjYEzZzQDcK";

pub fn page_text(url: String) -> Result<String, reqwest::Error> {
  reqwest::get(&url)?.text()
}

pub fn stream_url(text: String) -> String {
  let re = &Regex::new(r"soundcloud://sounds:(\d+)")
    .unwrap()
    .captures(&text)
    .unwrap()[0];
  let vector: Vec<&str> = re.split(":").collect();
  let stream_id: &str = &vector.last().unwrap();

  format!(
    "https://api.soundcloud.com/i1/tracks/{}/streams?client_id={}",
    stream_id, CLIENT_ID
  )
}

pub fn sound_link(url: String) -> Result<String, reqwest::Error> {
  let response: HashMap<String, String> = reqwest::get(&url)?.json()?;
  Ok(response["http_mp3_128_url"].to_string())
}

pub fn download(url: String) -> () {
  let mut file_get = reqwest::get(&url).expect("GG");
  let mut out = File::create("test.mp3").expect("GG 4!");
  io::copy(&mut file_get, &mut out).expect("gg");
}