// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::{Receiver, Sender};
use {defmt_rtt as _, panic_probe as _};

use crate::at::*;
use crate::channel::{CAP, Msg};
use crate::hexa_config::*;

#[embassy_executor::task]
pub async fn at_task(
    dispatcher: &'static AtDispatcher,
    spawner: Spawner,
    at_rx: Receiver<'static, Cs, Msg, CAP>,
    at_tx: Sender<'static, Cs, Msg, CAP>,
    usb_tx: Sender<'static, Cs, Msg, CAP>,
    rgb_tx: Sender<'static, Cs, Msg, CAP>,
    dds_tx: Sender<'static, Cs, Msg, CAP>,
) {
    info!("Starting AT task");
    loop {
        match at_rx.receive().await {
            Msg::AtRxLine(line) => {
                if let Some(e) = dispatcher.dispatch(spawner, at_tx, rgb_tx, dds_tx, &line) {
                    let compiled = compile_at_error(get_empty_id(), e);
                    error!("Dispatch error: {:?}", compiled.as_str());
                    usb_tx.send(Msg::UsbTxLine(compiled)).await;
                }
            }
            Msg::AtCmdOutput(at_command) => {
                let compiled = at_command.compile();
                info!("Sending output: {}", compiled.as_str());
                usb_tx.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::Done(msg_id) => {
                let compiled = compile_at_done(msg_id);
                info!("Sending done: {}", compiled.as_str());
                usb_tx.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::Err(msg_id, e) => {
                let compiled = compile_at_error(msg_id, e);
                error!("Sending error: {}", compiled.as_str());
                usb_tx.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::ErrWOCommand(e) => {
                let compiled = compile_at_error(get_empty_id(), e);
                error!("Sending error: {}", compiled.as_str());
                usb_tx.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::SetDdsAvailable(status) => {
                set_dds_available(status);
            }
            _ => {}
        }
    }
}
