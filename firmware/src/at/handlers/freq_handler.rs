// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_executor::Spawner;

use crate::DDS_CH;
use crate::at::{AtCommand, AtHandler};
use crate::channel::*;
use crate::error::Error;
use crate::hexa_config::*;

pub struct FreqHandler;

impl AtHandler for FreqHandler {
    fn handle(&self, spawner: Spawner, at_command: AtCommand) -> Option<Error> {
        if is_dds_available() {
            info!("Setting FREQ: {:?}", at_command.id.as_str());
            let _ = spawner.spawn(freq_task(at_command));
            None
        } else {
            error!("DDS busy, cannot set FREQ ({})", at_command.id.as_str());
            Some(Error::DdsBusy)
        }
    }
}

#[embassy_executor::task]
async fn freq_task(at_command: AtCommand) {
    info!(
        "Sending FREQ command to DDS task: {}",
        at_command.id.as_str()
    );
    DDS_CH.send(Msg::FreqWithValue(at_command)).await;
    info!("FREQ command sent to DDS task");
}
