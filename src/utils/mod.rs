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

macro_rules! get_next {
  ($b_iter:ident) => {
    *($b_iter.next().into_result()
      .map_err(|e| my_err_of_err!(e)).unwrap());
  };
}

macro_rules! push_byte {
  ($bytes:ident, $buf:expr) => {
    let buf_ = $buf;
    let byte = u8::from_str_radix(unsafe {
        from_utf8_unchecked(&buf_)
    }, 16).map_err(|e| my_err_of_err!(e)).unwrap();
    $bytes.push(byte);
  };
}
pub fn url_decode_ascii(encoded: &[u8]) -> Cow<[u8]> {
  if !encoded.contains(&b'%') {
    return Cow::Borrowed(encoded);
  }


  // URL Decode
  let mut bytes = vec![];
  {
    let mut b_iter = encoded.iter();

    while let Some(&b) = b_iter.next() {
      if b == b'%' {
        push_byte!(bytes, [get_next!(b_iter), get_next!(b_iter)]);
      } else {
        bytes.push(b);
      }
    }
  }

  bytes.shrink_to_fit();
  bytes.into()
}

pub fn url_decode_utf8(encoded: &[u8]) -> Cow<[u8]> {
  if !unsafe { from_utf8_unchecked(encoded) }.contains("\\u") {
    return Cow::Borrowed(encoded);
  }

  // URL Decode
  let mut bytes = vec![];
  {
    let mut b_iter = encoded.iter();

    while let Some(&b) = b_iter.next() {
      if b == b'\\' && get_next!(b_iter) == b'u' {
        push_byte!(bytes, [
            get_next!(b_iter), get_next!(b_iter),
            get_next!(b_iter), get_next!(b_iter)
          ]);
      } else {
        bytes.push(b);
      }
    }
  }

  bytes.shrink_to_fit();
  bytes.into()
}