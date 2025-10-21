// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::info;
use heapless::{String, Vec};

use crate::error::Error;

pub struct AtCommand {
    pub id: String<16>,
    pub name: String<16>,
    pub params: Vec<String<16>, 8>,
    pub is_query: bool,
}

impl AtCommand {
    pub fn compile(&self) -> String<64> {
        let mut s = String::<64>::new();
        s.push_str("AT+").unwrap();
        s.push_str(self.name.as_str()).unwrap();
        if self.is_query {
            s.push('?').unwrap();
        } else {
            s.push('=').unwrap();
            s.push_str(self.id.as_str()).unwrap();
            if !self.params.is_empty() {
                for p in self.params.iter() {
                    s.push('#').unwrap();
                    s.push_str(p.as_str()).unwrap();
                }
            }
        }
        s
    }
}

pub fn parse(input: &str) -> Result<AtCommand, Error> {
    info!("Received input: {}", input);
    let input = input.trim();
    if !input.starts_with("AT+") {
        return Err(Error::InvalidCommand);
    }
    let cmd = &input[3..];
    if let Some(eq_pos) = cmd.find('=') {
        let (name, param_str) = cmd.split_at(eq_pos);
        let mut id: String<16> = {
            let mut id = String::<16>::new();
            id.push_str("0").map_err(|_| Error::InvalidUtf8)?;
            id
        };
        let param_str = &param_str[1..];
        let mut params = Vec::<String<16>, 8>::new();
        for (i, p) in param_str
            .split('#')
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .enumerate()
        {
            info!("Parsing param: {}", p);
            let mut s_hl = String::<16>::new();
            s_hl.push_str(p).map_err(|_| Error::InvalidUtf8)?;
            if i == 0 {
                id = s_hl;
            } else {
                params.push(s_hl).map_err(|_| Error::ParamCount)?;
            }
        }
        Ok(AtCommand {
            id,
            name: {
                let mut n = String::<16>::new();
                n.push_str(name).map_err(|_| Error::InvalidUtf8)?;
                n
            },
            params,
            is_query: false,
        })
    } else if let Some(name) = cmd.strip_suffix('?') {
        Ok(AtCommand {
            id: {
                let mut id = String::<16>::new();
                id.push_str("0").map_err(|_| Error::InvalidUtf8)?;
                id
            },
            name: {
                let mut n = String::<16>::new();
                n.push_str(name).map_err(|_| Error::InvalidUtf8)?;
                n
            },
            params: Vec::<String<16>, 8>::new(),
            is_query: true,
        })
    } else {
        Ok(AtCommand {
            id: {
                let mut id = String::<16>::new();
                id.push_str("0").map_err(|_| Error::InvalidUtf8)?;
                id
            },
            name: {
                let mut n = String::<16>::new();
                n.push_str(cmd).map_err(|_| Error::InvalidUtf8)?;
                n
            },
            params: Vec::<String<16>, 8>::new(),
            is_query: false,
        })
    }
}

pub fn get_empty_id() -> String<16> {
    let mut id = String::<16>::new();
    id.push_str("0").unwrap();
    id
}

pub fn compile_at_done(id: String<16>) -> String<64> {
    let at_cmd = AtCommand {
        id,
        name: {
            let mut n = String::<16>::new();
            n.push_str("DONE").unwrap();
            n
        },
        params: Vec::<String<16>, 8>::new(),
        is_query: false,
    };
    at_cmd.compile()
}

pub fn compile_at_error(id: String<16>, e: Error) -> String<64> {
    let at_cmd = AtCommand {
        id,
        name: {
            let mut n = String::<16>::new();
            n.push_str("ERROR").unwrap();
            n
        },
        params: {
            let mut p = Vec::<String<16>, 8>::new();
            let mut code = String::<16>::new();
            code.push_str(e.code()).unwrap();
            p.push(code).unwrap();
            p
        },
        is_query: false,
    };
    at_cmd.compile()
}
