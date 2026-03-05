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
    rgb_led.set_rgb(0, 0, 255).await;
    info!("RGB set to blue");
    embassy_time::Timer::after_secs(1).await;
    rgb_led.set_rgb(0, 255, 0).await;
    info!("RGB set to green");
    embassy_time::Timer::after_secs(1).await;
    rgb_led.set_rgb(255, 0, 0).await;
    info!("RGB set to red");
    embassy_time::Timer::after_secs(1).await;
    rgb_led.set_rgb(0, 0, 0).await;
    info!("RGB set to off");
    embassy_time::Timer::after_secs(1).await;
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
