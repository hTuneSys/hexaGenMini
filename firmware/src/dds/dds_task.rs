// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use crate::AT_CH;
use crate::DDS_CH;
use crate::channel::*;
use crate::dds::Ad985x;
use crate::error::Error;

#[embassy_executor::task]
pub async fn dds_task(mut ad985x: Ad985x) {
    info!("Starting DDS task");
    loop {
        if let Msg::FreqWithValue(at_command) = DDS_CH.receive().await {
            info!(
                "Received FREQ command in DDS task: {}",
                at_command.id.as_str()
            );

            info!("Parsing FREQ command parameters");
            if at_command.params.len() != 2 {
                error!(
                    "FREQ command requires 3 parameters, got {}",
                    at_command.params.len()
                );
                AT_CH.send(Msg::Err(at_command.id, Error::DdsBusy)).await;
                return;
            }
            info!("Parameters parsed successfully");
            info!("Extracting frequency and time_ms values");
            let freq = match at_command.params[0].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid frequency value: {}", at_command.id.as_str());
                    AT_CH.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };
            info!("Frequency value extracted: {}", freq);
            let time_ms = match at_command.params[1].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid time_ms value: {}", at_command.id.as_str());
                    AT_CH.send(Msg::Err(at_command.id, Error::ParamValue)).await;
                    return;
                }
            };
            info!("time_ms value extracted: {}", time_ms);
            info!("Setting Device Available to false");
            AT_CH.send(Msg::SetDdsAvailable(false)).await;
            info!(
                "Setting FREQ to ({}) over {} ms",
                at_command.id.as_str(),
                time_ms
            );
            info!("Starting frequency set...");
            let err = ad985x.set_freq(freq, time_ms).await;
            info!("Frequency set complete.");
            AT_CH.send(Msg::SetDdsAvailable(true)).await;
            info!("Setting Device Available to true");
            if let Some(err) = err {
                error!("Error setting FREQ: {:?}", err.code());
                AT_CH.send(Msg::Err(at_command.id, err)).await;
                info!("Error sent");
            } else {
                info!("FREQ set successfully");
                AT_CH.send(Msg::Done(at_command.id)).await;
                info!("Done sent");
            }
        }
    }
}
