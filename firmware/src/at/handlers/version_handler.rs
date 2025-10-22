// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::error;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;
use heapless::{String, Vec};

use crate::at::*;
use crate::channel::{CAP, Msg};
use crate::error::Error;
use crate::hexa_config::CONF_VERSION;

pub struct VersionHandler;

impl AtHandler for VersionHandler {
    fn handle(
        &self,
        spawner: Spawner,
        at_tx: Sender<'static, Cs, Msg, CAP>,
        at_command: AtCommand,
    ) -> Option<Error> {
        if at_command.is_query {
            let _ = spawner.spawn(version_task(at_tx));
            None
        } else {
            error!("VERSION command is a query only");
            Some(Error::NotAQuery)
        }
    }
}

#[embassy_executor::task]
async fn version_task(at_tx: Sender<'static, Cs, Msg, CAP>) {
    let mut name = String::<16>::new();
    name.push_str("VERSION").unwrap();

    let mut params = Vec::<String<16>, 8>::new();
    let version_param = String::<16>::try_from(CONF_VERSION).unwrap();
    params.push(version_param).ok();

    let at_command = AtCommand {
        id: get_empty_id(),
        name,
        params,
        is_query: false,
    };
    at_tx.send(Msg::AtCmdOutput(at_command)).await;
}
