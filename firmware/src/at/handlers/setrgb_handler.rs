// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::error;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;
use heapless::String;

use crate::at::AtHandler;
use crate::channel::{CAP, Msg};
use crate::error::Error;

pub struct SetRgbHandler;

impl AtHandler for SetRgbHandler {
    fn handle(
        &self,
        spawner: Spawner,
        rgb_tx: Sender<'static, Cs, Msg, CAP>,
        params: &[String<16>],
        _is_query: bool,
    ) -> Option<Error> {
        if params.len() != 3 {
            error!("SETRGB command requires 3 parameters, got {}", params.len());
            return Some(Error::ParamCount);
        }
        // check params are u8
        let r = match params[0].parse::<u8>() {
            Ok(v) => v,
            Err(_) => return Some(Error::ParamValue),
        };
        let g = match params[1].parse::<u8>() {
            Ok(v) => v,
            Err(_) => return Some(Error::ParamValue),
        };
        let b = match params[2].parse::<u8>() {
            Ok(v) => v,
            Err(_) => return Some(Error::ParamValue),
        };
        let _ = spawner.spawn(setrgb_task(rgb_tx, r, g, b));
        None
    }
}

#[embassy_executor::task]
async fn setrgb_task(rgb_tx: Sender<'static, Cs, Msg, CAP>, r: u8, g: u8, b: u8) {
    rgb_tx.send(Msg::RgbWithValue(r, g, b)).await;
}
