// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use cortex_m::peripheral::SCB;
use embassy_executor::Spawner;

use crate::at::*;
use crate::error::Error;

pub struct ResetHandler;

impl AtHandler for ResetHandler {
    fn handle(&self, _spawner: Spawner, _at_command: AtCommand) -> Option<Error> {
        SCB::sys_reset();
    }
}
