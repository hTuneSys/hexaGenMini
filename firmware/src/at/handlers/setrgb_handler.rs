// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use embassy_executor::Spawner;

use crate::RGB_CH;
use crate::at::{AtCommand, AtHandler};
use crate::channel::*;
use crate::error::Error;

pub struct SetRgbHandler;

impl AtHandler for SetRgbHandler {
    fn handle(&self, spawner: Spawner, at_command: AtCommand) -> Option<Error> {
        let _ = spawner.spawn(setrgb_task(at_command));
        None
    }
}

#[embassy_executor::task]
async fn setrgb_task(at_command: AtCommand) {
    RGB_CH.send(Msg::RgbWithValue(at_command)).await;
}
