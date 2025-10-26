// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

use crate::AT_CH;
use crate::USB_CH;
use crate::at::*;
use crate::channel::*;
use crate::hexa_config::*;

#[embassy_executor::task]
pub async fn at_task(dispatcher: &'static AtDispatcher, spawner: Spawner) {
    info!("Starting AT task");
    loop {
        match AT_CH.receive().await {
            Msg::AtRxLine(line) => {
                if let Some(e) = dispatcher.dispatch(spawner, &line) {
                    let compiled = compile_at_error(get_empty_id(), e);
                    error!("Dispatch error: {:?}", compiled.as_str());
                    USB_CH.send(Msg::UsbTxLine(compiled)).await;
                }
            }
            Msg::AtCmdOutput(at_command) => {
                let compiled = at_command.compile();
                info!("Sending output: {}", compiled.as_str());
                USB_CH.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::Done(msg_id) => {
                let compiled = compile_at_done(msg_id);
                info!("Sending done: {}", compiled.as_str());
                USB_CH.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::Err(msg_id, e) => {
                let compiled = compile_at_error(msg_id, e);
                error!("Sending error: {}", compiled.as_str());
                USB_CH.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::SetDdsAvailable(status) => {
                set_dds_available(status);
            }
            _ => {}
        }
    }
}
