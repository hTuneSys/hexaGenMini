// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::info;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::{Receiver, Sender};
use heapless::{String, Vec};
use {defmt_rtt as _, panic_probe as _};

use crate::at::AtCommand;
use crate::channel::MsgDirection;
use crate::channel::{CAP, Msg};
use crate::dds::Ad985x;
use crate::error::Error;
use crate::hexa_config::*;

#[embassy_executor::task]
pub async fn dds_task(
    mut ad985x: Ad985x,
    dds_rx: Receiver<'static, Cs, Msg, CAP>,
    at_tx: Sender<'static, Cs, Msg, CAP>,
) {
    info!("Starting DDS task");
    loop {
        if let Msg::FreqWithValue(id, freq, time_ms) = dds_rx.receive().await {
            if is_dds_available() {
                set_dds_status(true);
                info!("Setting FREQ to ({}) over {} ms", id.as_str(), time_ms);
                if let Some(err) = ad985x.set_freq(freq, time_ms).await {
                    info!("Error setting FREQ: {:?}", err.code());
                    at_tx.send(Msg::Err(err)).await;
                } else {
                    let mut name = String::<16>::new();
                    name.push_str("FREQ").unwrap();

                    let mut params = Vec::<String<16>, 8>::new();
                    let id_param = String::<16>::try_from(id).unwrap();
                    params.push(id_param).ok();
                    let done_param = String::<16>::try_from("DONE").unwrap();
                    params.push(done_param).ok();

                    let compiled = AtCommand {
                        name,
                        params,
                        is_query: false,
                    }
                    .compile();
                    at_tx.send(Msg::AtCmd(MsgDirection::Output, compiled)).await;
                }
                set_dds_status(false);
            } else {
                info!("DDS busy, cannot set FREQ ({})", id.as_str());
                at_tx.send(Msg::Err(Error::DdsBusy)).await;
            }
        }
    }
}
