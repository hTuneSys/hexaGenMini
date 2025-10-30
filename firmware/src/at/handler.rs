// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use embassy_executor::Spawner;
use heapless::{LinearMap, String};

use crate::at::*;
use crate::error::Error;

pub trait AtHandler {
    fn handle(&self, spawner: Spawner, at_command: AtCommand) -> Option<Error>;
}

pub enum Handler {
    Version(VersionHandler),
    SetRgb(SetRgbHandler),
    Reset(ResetHandler),
    FwUpdate(FwUpdateHandler),
    Freq(FreqHandler),
    Operation(OperationHandler),
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

    map.insert(
        String::<16>::try_from("FREQ").unwrap(),
        Handler::Freq(FreqHandler),
    )
    .ok();

    map.insert(
        String::<16>::try_from("OPERATION").unwrap(),
        Handler::Operation(OperationHandler),
    )
    .ok();

    map
}
