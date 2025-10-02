// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

#![allow(dead_code)]
use core::fmt::Write;

use heapless::String;

mod parser;
pub use parser::parse;

mod handler;
pub use handler::AtHandler;

mod commands;

/// AT command dispatcher
pub struct AtDispatcher {
    handlers: heapless::LinearMap<heapless::String<16>, crate::at::commands::Handler, 8>,
}

impl AtDispatcher {
    /// Create a new dispatcher with all registered handlers
    pub fn new() -> Self {
        Self {
            handlers: commands::register_all(),
        }
    }

    /// Dispatch an AT command string to the appropriate handler
    pub fn dispatch(&self, input: &str) -> String<64> {
        match parse(input) {
            Ok(cmd) => match self.handlers.get(&cmd.name) {
                Some(handler) => {
                    let result = match handler {
                        crate::at::commands::Handler::Version(h) => {
                            h.handle(&cmd.params, cmd.is_query)
                        }
                        crate::at::commands::Handler::Sum(h) => h.handle(&cmd.params, cmd.is_query),
                    };
                    match result {
                        Ok(resp) => {
                            let encoded = crate::b64::encode(&resp);
                            let mut out = String::<64>::new();
                            write!(out, "AT+OK={}", encoded).unwrap();
                            out
                        }
                        Err(e) => {
                            let msg = e.description();
                            let encoded = crate::b64::encode(msg);
                            let mut out = String::<64>::new();
                            write!(out, "AT+ERROR={}", encoded).unwrap();
                            out
                        }
                    }
                }
                None => {
                    let msg = crate::error::Error::UnknownCommand.description();
                    let encoded = crate::b64::encode(msg);
                    let mut out = String::<64>::new();
                    write!(out, "AT+ERROR={}", encoded).unwrap();
                    out
                }
            },
            Err(e) => {
                let msg = e.description();
                let encoded = crate::b64::encode(msg);
                let mut out = String::<64>::new();
                write!(out, "AT+ERROR={}", encoded).unwrap();
                out
            }
        }
    }
}
