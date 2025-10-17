// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::info;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::{Receiver, Sender};
use {defmt_rtt as _, panic_probe as _};

use crate::channel::{CAP, Msg};
use crate::rgb::RgbLed;

#[embassy_executor::task]
pub async fn rgb_task(
    mut rgb_led: RgbLed,
    rgb_rx: Receiver<'static, Cs, Msg, CAP>,
    at_tx: Sender<'static, Cs, Msg, CAP>,
) {
    info!("Starting RGB task");
    loop {
        if let Msg::RgbWithValue(r, g, b) = rgb_rx.receive().await {
            info!("Setting RGB to ({}, {}, {})", r, g, b);
            rgb_led.set_rgb(r, g, b).await;
            info!("RGB set");
            info!("Sending OK from RGB task");
            at_tx.send(Msg::Ok).await;
        }
    }
}
