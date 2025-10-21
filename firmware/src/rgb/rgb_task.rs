// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::{Receiver, Sender};
use {defmt_rtt as _, panic_probe as _};

use crate::channel::{CAP, Msg};
use crate::error::Error;
use crate::rgb::RgbLed;

#[embassy_executor::task]
pub async fn rgb_task(
    mut rgb_led: RgbLed,
    rgb_rx: Receiver<'static, Cs, Msg, CAP>,
    at_tx: Sender<'static, Cs, Msg, CAP>,
) {
    info!("Starting RGB task");
    loop {
        if let Msg::RgbWithValue(at_command) = rgb_rx.receive().await {
            if at_command.params.len() != 3 {
                error!(
                    "SETRGB command {} requires 3 parameters, got {}",
                    at_command.id.as_str(),
                    at_command.params.len()
                );
                at_tx.send(Msg::Err(at_command.id, Error::ParamCount)).await;
                return;
            }
            // check params are u8
            let r = match at_command.params[0].parse::<u8>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid R value: {}", at_command.id.as_str());
                    at_tx.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };
            let g = match at_command.params[1].parse::<u8>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid G value: {}", at_command.id.as_str());
                    at_tx.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };
            let b = match at_command.params[2].parse::<u8>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid B value: {}", at_command.id.as_str());
                    at_tx.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };
            info!("Setting RGB to ({}, {}, {})", r, g, b);
            rgb_led.set_rgb(r, g, b).await;
            info!("RGB set");
            info!("Sending DONE from RGB task");
            at_tx.send(Msg::Done(at_command.id)).await;
        }
    }
}
