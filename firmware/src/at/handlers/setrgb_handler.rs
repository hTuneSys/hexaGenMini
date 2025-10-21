// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;

use crate::at::{AtCommand, AtHandler};
use crate::channel::{CAP, Msg};
use crate::error::Error;

pub struct SetRgbHandler;

impl AtHandler for SetRgbHandler {
    fn handle(
        &self,
        spawner: Spawner,
        rgb_tx: Sender<'static, Cs, Msg, CAP>,
        at_command: AtCommand,
    ) -> Option<Error> {
        let _ = spawner.spawn(setrgb_task(rgb_tx, at_command));
        None
    }
}

#[embassy_executor::task]
async fn setrgb_task(rgb_tx: Sender<'static, Cs, Msg, CAP>, at_command: AtCommand) {
    rgb_tx.send(Msg::RgbWithValue(at_command)).await;
}
