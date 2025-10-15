// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

//use base64ct::{Base64, Encoding};
use heapless::String;

use crate::error::Error;

pub fn b64_decode(input: &str) -> Result<String<64>, Error> {
    // let mut buf = [0u8; 64];
    // match Base64::decode(input, &mut buf) {
    //     Ok(decoded) => match core::str::from_utf8(decoded) {
    //         Ok(s) => {
    //             let mut out = String::<64>::new();
    //             out.push_str(s).map_err(|_| Error::InvalidUtf8)?;
    //             Ok(out)
    //         }
    //         Err(_) => Err(Error::InvalidUtf8),
    //     },
    //     Err(_) => Err(Error::InvalidBase64),
    // }

    let mut out = String::<64>::new();
    out.push_str(input).map_err(|_| Error::InvalidUtf8)?;
    Ok(out)
}

pub fn b64_encode(input: &str) -> String<64> {
    // let mut buf = [0u8; 64];
    // let encoded = Base64::encode(input.as_bytes(), &mut buf).unwrap();
    // let mut out = String::<64>::new();
    // out.push_str(encoded).unwrap();
    // out

    let mut out = String::<64>::new();
    out.push_str(input).unwrap();
    out
}
