// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;

use crate::at::{AtCommand, AtHandler};
use crate::channel::{CAP, Msg};
use crate::error::Error;
use crate::hexa_config::*;

pub struct FreqHandler;

impl AtHandler for FreqHandler {
    fn handle(
        &self,
        spawner: Spawner,
        dds_tx: Sender<'static, Cs, Msg, CAP>,
        at_command: AtCommand,
    ) -> Option<Error> {
        if is_dds_available() {
            info!("Setting FREQ: {:?}", at_command.id.as_str());
            let _ = spawner.spawn(freq_task(dds_tx, at_command));
            None
        } else {
            error!("DDS busy, cannot set FREQ ({})", at_command.id.as_str());
            Some(Error::DdsBusy)
        }
    }
}

#[embassy_executor::task]
async fn freq_task(dds_tx: Sender<'static, Cs, Msg, CAP>, at_command: AtCommand) {
    dds_tx.send(Msg::FreqWithValue(at_command)).await;
}
