// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;
use heapless::{LinearMap, String};

use crate::at::handler::{AtHandler, Handler, register_all};
use crate::at::parse;
use crate::channel::{CAP, Msg};
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

    pub fn dispatch(
        &self,
        spawner: Spawner,
        at_tx: Sender<'static, Cs, Msg, CAP>,
        rgb_tx: Sender<'static, Cs, Msg, CAP>,
        input: &str,
    ) -> Option<Error> {
        match parse(input) {
            Ok(cmd) => match self.handlers.get(&cmd.name) {
                Some(handler) => match handler {
                    Handler::Version(h) => {
                        info!("Dispatching VERSION command");
                        h.handle(spawner, at_tx, &cmd.params, cmd.is_query)
                    }
                    Handler::SetRgb(h) => {
                        info!("Dispatching SETRGB command");
                        h.handle(spawner, rgb_tx, &cmd.params, cmd.is_query)
                    }
                    Handler::Reset(h) => {
                        info!("Dispatching RESET command");
                        h.handle(spawner, at_tx, &cmd.params, cmd.is_query)
                    }
                    Handler::FwUpdate(h) => {
                        info!("Dispatching FWUPDATE command");
                        h.handle(spawner, at_tx, &cmd.params, cmd.is_query)
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
