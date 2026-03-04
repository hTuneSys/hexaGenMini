// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::info;

#[embassy_executor::task]
pub async fn fwupdate_task() {
    info!("Entering BOOTSEL mode for firmware update");
    embassy_time::Timer::after_millis(100).await;
    embassy_rp::rom_data::reset_to_usb_boot(0, 0);
}
