// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use cortex_m::peripheral::SCB;
use defmt::{error, info};
use embassy_executor::Spawner;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

use hexa_tune_proto_embedded::command::HexaCommand;
use hexa_tune_proto_embedded::HexaError;

use crate::AT_CH;
use crate::USB_CH;
use crate::at::*;
use crate::channel::*;
use crate::error::FirmwareError;
use crate::hexa_config::*;

#[embassy_executor::task]
pub async fn at_task(spawner: Spawner) {
    info!("Starting AT task");
    let mut last_operation_status: String<64> = String::new();
    loop {
        match AT_CH.receive().await {
            Msg::AtRxLine(line) => {
                if let Err((id, e)) = dispatch_and_spawn(spawner, line.as_bytes()) {
                    let compiled = encode_error_response(id, &e);
                    error!("Dispatch error: {:?}", compiled.as_str());
                    USB_CH.send(Msg::UsbTxLine(compiled)).await;
                }
            }
            Msg::AtCmdResponse(line) => {
                info!("Sending response: {}", line.as_str());
                USB_CH.send(Msg::UsbTxLine(line)).await;
            }
            Msg::Done(msg_id) => {
                let compiled = encode_done(msg_id);
                info!("Sending done: {}", compiled.as_str());
                USB_CH.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::Err(msg_id, e) => {
                let compiled = encode_error_response(msg_id, &e);
                error!("Sending error: {}", compiled.as_str());
                USB_CH.send(Msg::UsbTxLine(compiled)).await;
            }
            Msg::SetDdsAvailable(status) => {
                set_dds_available(status);
            }
            Msg::SetOperationStatus(status) => {
                last_operation_status = status;
            }
            Msg::GetOperationStatus => {
                USB_CH
                    .send(Msg::UsbTxLine(last_operation_status.clone()))
                    .await;
            }
            _ => {}
        }
    }
}

fn dispatch_and_spawn(spawner: Spawner, payload: &[u8]) -> Result<(), (u32, FirmwareError)> {
    let cmd = dispatch_at_payload(payload).map_err(|e| (0u32, e))?;
    let id = command_id(&cmd);

    match cmd {
        HexaCommand::VersionQuery => {
            info!("Dispatching VERSION query");
            spawner.spawn(version_task()).ok();
        }
        HexaCommand::SetRgb { id, r, g, b } => {
            info!("Dispatching SETRGB command");
            spawner.spawn(setrgb_task(id, r, g, b)).ok();
        }
        HexaCommand::Reset { .. } => {
            info!("Dispatching RESET command");
            SCB::sys_reset();
        }
        HexaCommand::FwUpdate { .. } => {
            info!("Dispatching FWUPDATE command");
            spawner.spawn(fwupdate_task()).ok();
        }
        HexaCommand::Freq { id, freq, time_ms } => {
            if !is_dds_available() {
                error!("DDS busy, cannot set FREQ");
                return Err((id, FirmwareError::Hexa(HexaError::DdsBusy)));
            }
            info!("Dispatching FREQ command");
            spawner.spawn(freq_task(id, freq, time_ms)).ok();
        }
        HexaCommand::Operation { id, sub } => {
            if !is_dds_available() {
                error!("DDS busy, cannot set OPERATION");
                return Err((id, FirmwareError::Hexa(HexaError::DdsBusy)));
            }
            info!("Dispatching OPERATION command");
            spawner.spawn(operation_task(id, sub)).ok();
        }
        HexaCommand::OperationQuery => {
            info!("Dispatching OPERATION query");
            spawner.spawn(operation_status_task()).ok();
        }
        _ => {
            return Err((id, FirmwareError::Hexa(HexaError::UnknownCommand)));
        }
    }
    Ok(())
}

fn command_id(cmd: &HexaCommand) -> u32 {
    match cmd {
        HexaCommand::SetRgb { id, .. }
        | HexaCommand::Reset { id }
        | HexaCommand::FwUpdate { id }
        | HexaCommand::Freq { id, .. }
        | HexaCommand::Operation { id, .. } => *id,
        _ => 0,
    }
}
