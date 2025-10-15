// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Sender;
use heapless::{LinearMap, String};

use crate::at::*;
use crate::channel::{CAP, Msg};
use crate::error::Error;

pub trait AtHandler {
    fn handle(
        &self,
        spawner: Spawner,
        tx: Sender<'static, Cs, Msg, CAP>,
        params: &[String<16>],
        is_query: bool,
    ) -> Option<Error>;
}

pub enum Handler {
    Version(VersionHandler),
    SetRgb(SetRgbHandler),
    Reset(ResetHandler),
    FwUpdate(FwUpdateHandler),
}

pub fn register_all() -> LinearMap<String<16>, Handler, 8> {
    let mut map: LinearMap<String<16>, Handler, 8> = LinearMap::new();

    map.insert(
        String::<16>::try_from("VERSION").unwrap(),
        Handler::Version(VersionHandler),
    )
    .ok();

    map.insert(
        String::<16>::try_from("SETRGB").unwrap(),
        Handler::SetRgb(SetRgbHandler),
    )
    .ok();

    map.insert(
        String::<16>::try_from("RESET").unwrap(),
        Handler::Reset(ResetHandler),
    )
    .ok();

    map.insert(
        String::<16>::try_from("FWUPDATE").unwrap(),
        Handler::FwUpdate(FwUpdateHandler),
    )
    .ok();

    map
}
