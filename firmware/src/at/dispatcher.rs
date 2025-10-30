// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_executor::Spawner;
use heapless::{LinearMap, String};

use crate::at::handler::{AtHandler, Handler, register_all};
use crate::at::parse;
use crate::error::Error;

pub struct AtDispatcher {
    handlers: LinearMap<String<16>, Handler, 8>,
}

impl AtDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: register_all(),
        }
    }

    pub fn dispatch(&self, spawner: Spawner, input: &str) -> Option<Error> {
        match parse(input) {
            Ok(cmd) => match self.handlers.get(&cmd.name) {
                Some(handler) => match handler {
                    Handler::Version(h) => {
                        info!("Dispatching VERSION command");
                        h.handle(spawner, cmd)
                    }
                    Handler::SetRgb(h) => {
                        info!("Dispatching SETRGB command");
                        h.handle(spawner, cmd)
                    }
                    Handler::Reset(h) => {
                        info!("Dispatching RESET command");
                        h.handle(spawner, cmd)
                    }
                    Handler::FwUpdate(h) => {
                        info!("Dispatching FWUPDATE command");
                        h.handle(spawner, cmd)
                    }
                    Handler::Freq(h) => {
                        info!("Dispatching FREQ command");
                        h.handle(spawner, cmd)
                    }
                    Handler::Operation(h) => {
                        info!("Dispatching OPERATION command");
                        h.handle(spawner, cmd)
                    }
                },
                None => Some(Error::UnknownCommand),
            },
            Err(e) => {
                error!("Failed to parse command: {}", &e.code());
                Some(e)
            }
        }
    }
}
