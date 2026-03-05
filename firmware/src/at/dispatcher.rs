// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use hexa_tune_proto::at::{self, AtOp};
use hexa_tune_proto_embedded::command::HexaCommand;
use hexa_tune_proto_embedded::dispatch::resolve;

use crate::channel::MsgString;
use crate::error::FirmwareError;

/// Parse an AT payload and resolve it to a typed HexaCommand.
pub fn dispatch_at_payload(payload: &[u8]) -> Result<HexaCommand, FirmwareError> {
    let msg = at::parse(payload).map_err(FirmwareError::Proto)?;
    resolve(&msg).map_err(FirmwareError::Hexa)
}

/// Encode an AT response (name=id#params...) into a MsgString.
pub fn encode_response(name: &[u8], id: u32, params: &[&[u8]]) -> MsgString {
    let mut buf = [0u8; 64];
    if let Ok(n) = at::encode(name, id, AtOp::Response, params, &mut buf) {
        if let Ok(s) = core::str::from_utf8(&buf[..n]) {
            if let Ok(line) = MsgString::try_from(s) {
                return line;
            }
        }
    }
    MsgString::new()
}

/// Encode an AT+DONE=id response.
pub fn encode_done(id: u32) -> MsgString {
    encode_response(b"DONE", id, &[])
}

/// Encode an AT+ERROR=id#code response using u8 error code.
pub fn encode_error_response(id: u32, e: &FirmwareError) -> MsgString {
    let code = e.error_code();
    let mut code_buf = [0u8; 3];
    let code_len = u8_to_ascii(code, &mut code_buf);
    encode_response(b"ERROR", id, &[&code_buf[..code_len]])
}

/// Convert a u32 value to ASCII decimal bytes in a buffer.
pub fn u32_to_ascii_buf(val: u32, buf: &mut [u8; 10]) -> usize {
    if val == 0 {
        buf[0] = b'0';
        return 1;
    }
    let mut tmp = [0u8; 10];
    let mut n = val;
    let mut i = 10usize;
    while n > 0 {
        i -= 1;
        tmp[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    let len = 10 - i;
    buf[..len].copy_from_slice(&tmp[i..]);
    len
}

fn u8_to_ascii(val: u8, buf: &mut [u8; 3]) -> usize {
    if val >= 100 {
        buf[0] = b'0' + val / 100;
        buf[1] = b'0' + (val / 10) % 10;
        buf[2] = b'0' + val % 10;
        3
    } else if val >= 10 {
        buf[0] = b'0' + val / 10;
        buf[1] = b'0' + val % 10;
        2
    } else {
        buf[0] = b'0' + val;
        1
    }
}
