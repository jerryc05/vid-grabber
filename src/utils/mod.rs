#![allow(dead_code)]
#![allow(unused_macros)]

use std::borrow::Cow;
use std::ops::Try;
use std::str::from_utf8_unchecked;

use my_err::MyErr;

#[macro_use]
pub mod my_err;

#[inline]
pub const fn is_debug() -> bool {
  cfg!(debug_assertions)
}

#[macro_use]
macro_rules! static_assert {
  ($x:expr $(,)?) => {
    const _: [
      ();
      0 - !{ const ASSERT: bool = $x; ASSERT } as usize
    ] = [];
  };
}

#[inline]
pub fn url_decode(encoded: &[u8]) -> Cow<str> {
  unsafe {
    if !encoded.contains(&b'%') && !from_utf8_unchecked(encoded).contains("\\u") {
      return Cow::Borrowed(from_utf8_unchecked(encoded));
    }
  }

  let mut bytes = vec![];

  // URL Decode
  {
    let mut b_iter = encoded.iter();
    macro_rules! get_next {
      () => {
        *(b_iter.next().into_result()
          .map_err(|e| my_err_of_err!(e)).unwrap());
      };
    }
    macro_rules! push_byte {
      ($buf:expr) => {
        let buf_ = $buf;
        let byte = u8::from_str_radix(unsafe {
            from_utf8_unchecked(&buf_)
        }, 16).map_err(|e| my_err_of_err!(e)).unwrap();
        bytes.push(byte);
      };
    }

    while let Some(&b) = b_iter.next() {
      match b {
        b'%' => {
          push_byte!([get_next!(), get_next!()]);
        }
        _ => {
          bytes.push(b);
        }
      }
    }

    let temp = bytes.clone();
    b_iter = temp.as_slice().iter();
    bytes.clear();
    while let Some(&b) = b_iter.next() {
      match b {
        b'\\' if get_next!() == b'u' => {
          push_byte!([get_next!(), get_next!(), get_next!(), get_next!()]);
        }
        _ => {
          bytes.push(b);
        }
      }
    }
  }

  String::from_utf8(bytes)
      .map_err(|e| my_err_of_err!(e)).unwrap().into()
}