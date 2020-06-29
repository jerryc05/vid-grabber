#![feature(try_trait)]
#![feature(cow_is_borrowed)]


use std::io::{stdin, stdout, Write};
use std::ops::Try;
use std::str::from_utf8_unchecked;

use http_req::request;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde_json::{from_str, Value};

pub(crate) use utils::my_err::MyErr;
use vid_struct::VidStruct;

use crate::utils::url_decode_ascii;

mod vid_struct;
#[macro_use]
pub mod utils;

fn main() {
  let v_id = parse_v_id();
  let streaming_data = get_data(&v_id);
  let (formats, a_formats) = parse_formats_str(&streaming_data);
  let mut vec = vec![];
  parse_formats(a_formats, &mut vec);
  parse_formats(formats, &mut vec);
  for x in vec {
    println!("{:?}", x);
  }
}

fn parse_v_id() -> String {
  // read link
  let mut link = String::new();
  {
    println!("(For Example: https://www.youtube.com/watch?v=_0bwJM-uTd8)");
    print!("Youtube Link: ");
    stdout().flush().map_err(|e| my_err_of_err!(e)).unwrap();
    stdin().read_line(&mut link).map_err(|e| my_err_of_err!(e)).unwrap();
  }

  // parse v_id
  {
    lazy_static! {
    static ref RE: Regex = Regex::new(
          r#"(?:(?:youtu(?:\.be|be\.com)/)(?:watch\?(?:.*&)?v=|(?:embed|v)/))([^\?&"'>\n]+)"#
        ).map_err(|e| my_err_of_err!(e)).unwrap();
    }
    let cap: Captures = RE.captures_iter(&link).next().into_result()
        .map_err(|e| my_err_of_err!(e)).unwrap();
    let v_id = (&cap[1]).to_string();
    println!("---");
    println!("Video ID: {}", v_id);
    println!("---");
    v_id
  }
}

fn get_data(v_id: &str) -> String {
  let mut buffer = vec![];
  request::get(format!("https://www.youtube.com/get_video_info?video_id={}", v_id),
               &mut buffer).map_err(|e| my_err_of_err!(e)).unwrap();

  // Discard useless part
  let i = {
    let str_buf = unsafe { from_utf8_unchecked(&buffer) };
    str_buf.find("streamingData").into_result()
        .map_err(|e| my_err_of_err!(e)).unwrap()
  };

  unsafe {
    String::from_utf8_unchecked(
      url_decode_ascii(&buffer[i..]).into_owned())
  }
}

fn parse_formats_str(info: &str) -> (&str, &str) {
  let f = |s: &str| {
    let start = info.find(s).into_result()
        .map_err(|e| my_err_of_err!(e)).unwrap();
    let info2 = &info[start + s.len() + 2..];
    let end = info2.find(']').into_result()
        .map_err(|e| my_err_of_err!(e)).unwrap();
    &info2[..=end]
  };

  (f("\"formats"), f("\"adaptiveFormats"))
}

macro_rules! as_str {
  ($s:expr) => {
    $s.as_str().into_result().map_err(|e| my_err_of_err!(e)).unwrap()
  };
}

fn parse_formats(s: &str, vec: &mut Vec<VidStruct>) {
  let v: Value = from_str(s).map_err(|e| my_err_of_err!(e)).unwrap();
  if let Value::Array(map) = v {
    for item in map {
      vec.push(VidStruct {
        a_q: item["audioQuality"].as_str()
            .map(|s| s.to_string()),
        a_rate: item["audioSampleRate"].as_str()
            .map(|s| s.to_string()),
        v_q: item["qualityLabel"].as_str()
            .map(|s| s.to_string()),
        v_fps: item["fps"].as_u64(),
        bitrate: item["bitrate"].as_u64()
            .into_result().map_err(|e| my_err_of_err!(e)).unwrap(),
        mime: as_str!(item["mimeType"]).to_string(),
        url: as_str!(item["url"]).to_string(),
      })
    }
  }
}