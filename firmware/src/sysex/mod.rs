// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use heapless::Vec;

pub fn build_sysex<const N: usize>(payload: &str) -> Option<Vec<u8, N>> {
    let mut out: Vec<u8, N> = Vec::new();

    // SysEx start
    out.push(0xF0).ok()?;

    // Payload
    for b in payload.as_bytes() {
        out.push(*b).ok()?;
    }

    // SysEx end
    out.push(0xF7).ok()?;

    Some(out)
}

pub fn sysex_to_usb_midi_packets<const M: usize>(sysex: &[u8]) -> Vec<[u8; 4], M> {
    let mut out: Vec<[u8; 4], M> = Vec::new();

    let mut i = 0usize;
    while i < sysex.len() {
        let rem = sysex.len() - i;
        if rem >= 3 {
            if rem == 3 && sysex[sysex.len() - 1] == 0xF7 {
                out.push([0x07, sysex[i], sysex[i + 1], sysex[i + 2]]).ok(); // end with 3
                i += 3;
            } else {
                out.push([0x04, sysex[i], sysex[i + 1], sysex[i + 2]]).ok(); // start/continue
                i += 3;
            }
        } else if rem == 2 {
            out.push([0x06, sysex[i], sysex[i + 1], 0x00]).ok(); // end with 2
            i += 2;
        } else {
            out.push([0x05, sysex[i], 0x00, 0x00]).ok(); // end with 1
            i += 1;
        }
    }
    out
}

pub fn extract_sysex_payload(data: &[u8]) -> Option<Vec<u8, 512>> {
    let mut out: Vec<u8, 512> = Vec::new();

    for chunk in data.chunks_exact(4) {
        let cin = chunk[0] & 0x0F;
        let b1 = chunk[1];
        let b2 = chunk[2];
        let b3 = chunk[3];

        match cin {
            0x4 => {
                // SysEx continue/start (3 byte data)
                for &b in &[b1, b2, b3] {
                    if b != 0 {
                        out.push(b).ok();
                    }
                }
            }
            0x5 => {
                if b1 != 0 {
                    out.push(b1).ok();
                }
            } // end with 1
            0x6 => {
                for &b in &[b1, b2] {
                    if b != 0 {
                        out.push(b).ok();
                    }
                }
            } // end with 2
            0x7 => {
                for &b in &[b1, b2, b3] {
                    if b != 0 {
                        out.push(b).ok();
                    }
                }
            } // end with 3
            _ => {}
        }
    }

    if out.first().copied() == Some(0xF0) && out.last().copied() == Some(0xF7) {
        let mut payload: Vec<u8, 512> = Vec::new();
        payload.extend_from_slice(&out[1..out.len() - 1]).ok()?;
        Some(payload)
    } else {
        None
    }
}
