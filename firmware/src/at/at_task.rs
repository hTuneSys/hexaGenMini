// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::{Receiver, Sender};
use {defmt_rtt as _, panic_probe as _};

use crate::at::{AtDispatcher, compile_at_error, compile_at_ok};
use crate::channel::{CAP, Msg, MsgDirection};

#[embassy_executor::task]
pub async fn at_task(
    dispatcher: &'static AtDispatcher,
    spawner: Spawner,
    at_rx: Receiver<'static, Cs, Msg, CAP>,
    at_tx: Sender<'static, Cs, Msg, CAP>,
    usb_tx: Sender<'static, Cs, Msg, CAP>,
    rgb_tx: Sender<'static, Cs, Msg, CAP>,
) {
    info!("Starting AT task");
    loop {
        match at_rx.receive().await {
            Msg::AtCmd(MsgDirection::Input, line) => {
                if let Some(e) = dispatcher.dispatch(spawner, at_tx, usb_tx, rgb_tx, &line) {
                    error!("Dispatch error: {:?}", &e.code());
                    let compiled = compile_at_error(e);
                    usb_tx.send(Msg::UsbTxLine(compiled)).await;
                }
            }
            Msg::AtCmd(MsgDirection::Output, line) => {
                usb_tx.send(Msg::UsbTxLine(line)).await;
            }
            Msg::Ok => {
                let compiled = compile_at_ok();
                info!("Sending OK from AT task");
                usb_tx.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::Err(e) => {
                error!("Error Code: {}", &e.code());
                let compiled = compile_at_error(e);
                usb_tx.send(Msg::UsbTxLine(compiled)).await;
            }
            _ => {}
        }
    }
}
