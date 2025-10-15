// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use cortex_m::peripheral::SCB;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;
use heapless::String;

use crate::at::AtHandler;
use crate::channel::{CAP, Msg};
use crate::error::Error;

pub struct ResetHandler;

impl AtHandler for ResetHandler {
    fn handle(
        &self,
        _spawner: Spawner,
        _at_tx: Sender<'static, Cs, Msg, CAP>,
        _params: &[String<16>],
        _is_query: bool,
    ) -> Option<Error> {
        SCB::sys_reset();
    }
}
