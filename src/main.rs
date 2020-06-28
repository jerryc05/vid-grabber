#![feature(try_trait)]
#![feature(cow_is_borrowed)]

use std::borrow::Cow;
use std::io::{stdin, stdout, Write};
use std::ops::Try;
use std::str::from_utf8_unchecked;

use http_req::request;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

use utils::my_err::MyErr;

use crate::utils::url_decode;

#[macro_use]
pub mod utils;

fn main() {
  let v_id = parse_v_id();
  let streaming_data = get_data(&v_id);
  let (formats,a_formats) = parse_formats(&streaming_data);
  println!("{}\n---\n{}", formats,a_formats);
}

fn parse_v_id() -> String {
  // read link
  let mut link = "https://www.youtube.com/watch?v=_0bwJM-uTd8".to_string();//String::new();
  // {
  //   print!("Youtube Link:");
  //   stdout().flush().map_err(|e| my_err_of_err!(e)).unwrap();
  //   stdin().read_line(&mut link).map_err(|e| my_err_of_err!(e)).unwrap();
  // }

  // parse v_id
  {
    lazy_static! {
    static ref RE: Regex = Regex::new(
          r#"(?:(?:youtu(?:\.be|be\.com)/)(?:watch\?(?:.*&)?v=|(?:embed|v)/))([^\?&"'>]+)"#
        ).map_err(|e| my_err_of_err!(e)).unwrap();
    }
    let cap: Captures = RE.captures_iter(&link).next().into_result()
        .map_err(|e| my_err_of_err!(e)).unwrap();
    let v_id = (&cap[1]).to_string();
    println!("Video ID: {}", v_id);
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

  url_decode(&buffer[i..]).to_string()
}

fn parse_formats(info: &str) -> (&str, &str) {
  let f = |s| {
    let start = info.find(s).into_result()
        .map_err(|e| my_err_of_err!(e)).unwrap();
    let info2 = &info[start..];
    let end = info2.find(']').into_result()
        .map_err(|e| my_err_of_err!(e)).unwrap();
     &info2[..=end]
  };

  (f("\"formats"),f("\"adaptiveFormats"))
}