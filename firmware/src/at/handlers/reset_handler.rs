// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use cortex_m::peripheral::SCB;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;

use crate::at::*;
use crate::channel::{CAP, Msg};
use crate::error::Error;

pub struct ResetHandler;

impl AtHandler for ResetHandler {
    fn handle(
        &self,
        _spawner: Spawner,
        _at_tx: Sender<'static, Cs, Msg, CAP>,
        _at_command: AtCommand,
    ) -> Option<Error> {
        SCB::sys_reset();
    }
}
