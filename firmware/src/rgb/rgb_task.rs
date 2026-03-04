// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use crate::AT_CH;
use crate::RGB_CH;
use crate::channel::*;
use crate::rgb::RgbLed;

#[embassy_executor::task]
pub async fn rgb_task(mut rgb_led: RgbLed) {
    info!("Starting RGB task");
    loop {
        if let Msg::RgbSet { id, r, g, b } = RGB_CH.receive().await {
            info!("Setting RGB to ({}, {}, {})", r, g, b);
            rgb_led.set_rgb(r, g, b).await;
            info!("RGB set");
            info!("Sending DONE from RGB task");
            AT_CH.send(Msg::Done(id)).await;
        }
    }
}
