// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::info;
use heapless::{String, Vec};

/// Parsed AT command
pub struct ParsedAtCommand {
    pub name: heapless::String<16>,
    pub params: heapless::Vec<heapless::String<16>, 8>,
    pub is_query: bool,
}

/// Parse AT command string into name, params, and query flag
pub fn parse(input: &str) -> Result<ParsedAtCommand, crate::error::Error> {
    let input = input.trim();
    if !input.starts_with("AT+") {
        return Err(crate::error::Error::InvalidCommand);
    }
    let cmd = &input[3..];
    if let Some(eq_pos) = cmd.find('=') {
        let (name, param_str) = cmd.split_at(eq_pos);
        let param_str = &param_str[1..];
        let mut params = Vec::<String<16>, 8>::new();
        for p in param_str
            .split('#')
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
        {
            info!("Parsing param: {}", p);
            match crate::b64::decode(p) {
                Ok(decoded) => {
                    info!("Decoded param: {}", decoded.as_str());
                    let mut s_hl = String::<16>::new();
                    s_hl.push_str(&decoded)
                        .map_err(|_| crate::error::Error::InvalidUtf8)?;
                    params
                        .push(s_hl)
                        .map_err(|_| crate::error::Error::ParamCount)?;
                }
                Err(_) => {
                    info!("Base64 decode failed for param: {}", p);
                    return Err(crate::error::Error::InvalidBase64);
                }
            }
        }
        Ok(ParsedAtCommand {
            name: {
                let mut n = String::<16>::new();
                n.push_str(name)
                    .map_err(|_| crate::error::Error::InvalidUtf8)?;
                n
            },
            params,
            is_query: false,
        })
    } else if let Some(name) = cmd.strip_suffix('?') {
        Ok(ParsedAtCommand {
            name: {
                let mut n = String::<16>::new();
                n.push_str(name)
                    .map_err(|_| crate::error::Error::InvalidUtf8)?;
                n
            },
            params: Vec::<String<16>, 8>::new(),
            is_query: true,
        })
    } else {
        Ok(ParsedAtCommand {
            name: {
                let mut n = String::<16>::new();
                n.push_str(cmd)
                    .map_err(|_| crate::error::Error::InvalidUtf8)?;
                n
            },
            params: Vec::<String<16>, 8>::new(),
            is_query: false,
        })
    }
}
