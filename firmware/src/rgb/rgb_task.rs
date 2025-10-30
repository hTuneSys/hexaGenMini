// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use crate::AT_CH;
use crate::RGB_CH;
use crate::channel::*;
use crate::error::Error;
use crate::rgb::RgbLed;

#[embassy_executor::task]
pub async fn rgb_task(mut rgb_led: RgbLed) {
    info!("Starting RGB task");
    loop {
        if let Msg::RgbWithValue(at_command) = RGB_CH.receive().await {
            if at_command.params.len() != 3 {
                error!(
                    "SETRGB command {} requires 3 parameters, got {}",
                    at_command.id.as_str(),
                    at_command.params.len()
                );
                AT_CH.send(Msg::Err(at_command.id, Error::ParamCount)).await;
                return;
            }
            // check params are u8
            let r = match at_command.params[0].parse::<u8>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid R value: {}", at_command.id.as_str());
                    AT_CH.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };
            let g = match at_command.params[1].parse::<u8>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid G value: {}", at_command.id.as_str());
                    AT_CH.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };
            let b = match at_command.params[2].parse::<u8>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid B value: {}", at_command.id.as_str());
                    AT_CH.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };
            info!("Setting RGB to ({}, {}, {})", r, g, b);
            rgb_led.set_rgb(r, g, b).await;
            info!("RGB set");
            info!("Sending DONE from RGB task");
            AT_CH.send(Msg::Done(at_command.id)).await;
        }
    }
}
