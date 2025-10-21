// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::info;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;

use crate::at::*;
use crate::channel::{CAP, Msg};
use crate::error::Error;

pub struct FwUpdateHandler;

impl AtHandler for FwUpdateHandler {
    fn handle(
        &self,
        spawner: Spawner,
        _at_tx: Sender<'static, Cs, Msg, CAP>,
        _at_command: AtCommand,
    ) -> Option<Error> {
        info!("Firmware update requested - spawning BOOTSEL task");
        let _ = spawner.spawn(fwupdate_task());
        None
    }
}

#[embassy_executor::task]
async fn fwupdate_task() {
    info!("Entering BOOTSEL mode for firmware update");
    embassy_time::Timer::after_millis(100).await;
    embassy_rp::rom_data::reset_to_usb_boot(0, 0);
}
