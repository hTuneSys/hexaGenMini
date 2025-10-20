// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;
use heapless::String;

use crate::at::AtHandler;
use crate::channel::{CAP, Msg};
use crate::error::Error;
use crate::hexa_config::is_dds_available;

pub struct FreqHandler;

impl AtHandler for FreqHandler {
    fn handle(
        &self,
        spawner: Spawner,
        dds_tx: Sender<'static, Cs, Msg, CAP>,
        params: &[String<16>],
        _is_query: bool,
    ) -> Option<Error> {
        if params.len() != 3 {
            error!("FREQ command requires 3 parameters, got {}", params.len());
            return Some(Error::ParamCount);
        }
        let id = match params[0].parse::<String<16>>() {
            Ok(v) => v,
            Err(_) => return Some(Error::ParamValue),
        };
        let freq = match params[1].parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Some(Error::ParamValue),
        };
        let time_ms = match params[2].parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Some(Error::ParamValue),
        };
        if is_dds_available() {
            let _ = spawner.spawn(freq_task(dds_tx, id, freq, time_ms));
            None
        } else {
            info!("DDS busy, cannot set FREQ ({})", id.as_str());
            Some(Error::DdsBusy)
        }
    }
}

#[embassy_executor::task]
async fn freq_task(dds_tx: Sender<'static, Cs, Msg, CAP>, id: String<16>, freq: u32, time_ms: u32) {
    dds_tx.send(Msg::FreqWithValue(id, freq, time_ms)).await;
}
