// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::{Receiver, Sender};
use {defmt_rtt as _, panic_probe as _};

use crate::channel::{CAP, Msg};
use crate::dds::Ad985x;
use crate::error::Error;

#[embassy_executor::task]
pub async fn dds_task(
    mut ad985x: Ad985x,
    dds_rx: Receiver<'static, Cs, Msg, CAP>,
    at_tx: Sender<'static, Cs, Msg, CAP>,
) {
    info!("Starting DDS task");
    loop {
        if let Msg::FreqWithValue(at_command) = dds_rx.receive().await {
            if at_command.params.len() != 2 {
                error!(
                    "FREQ command requires 3 parameters, got {}",
                    at_command.params.len()
                );
                at_tx.send(Msg::Err(at_command.id, Error::DdsBusy)).await;
                return;
            }
            let freq = match at_command.params[0].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid frequency value: {}", at_command.id.as_str());
                    at_tx.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };
            let time_ms = match at_command.params[1].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid time_ms value: {}", at_command.id.as_str());
                    at_tx.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };

            at_tx.send(Msg::SetDdsAvailable(false)).await;
            info!(
                "Setting FREQ to ({}) over {} ms",
                at_command.id.as_str(),
                time_ms
            );
            let err = ad985x.set_freq(freq, time_ms).await;
            at_tx.send(Msg::SetDdsAvailable(true)).await;
            if let Some(err) = err {
                error!("Error setting FREQ: {:?}", err.code());
                at_tx.send(Msg::Err(at_command.id, err)).await;
            } else {
                at_tx.send(Msg::Done(at_command.id)).await;
            }
        }
    }
}
