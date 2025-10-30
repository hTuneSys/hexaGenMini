// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::error;
use embassy_executor::Spawner;
use heapless::{String, Vec};

use crate::AT_CH;
use crate::at::*;
use crate::channel::*;
use crate::error::Error;
use crate::hexa_config::CONF_VERSION;

pub struct VersionHandler;

impl AtHandler for VersionHandler {
    fn handle(&self, spawner: Spawner, at_command: AtCommand) -> Option<Error> {
        if at_command.is_query {
            let _ = spawner.spawn(version_task());
            None
        } else {
            error!("VERSION command is a query only");
            Some(Error::NotAQuery)
        }
    }
}

#[embassy_executor::task]
async fn version_task() {
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
    AT_CH.send(Msg::AtCmdOutput(at_command)).await;
}
