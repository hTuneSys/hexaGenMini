// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::{error, info};
use embassy_executor::Spawner;

use crate::AT_CH;
use crate::DDS_CH;
use crate::at::{AtCommand, AtHandler};
use crate::channel::*;
use crate::error::Error;
use crate::hexa_config::*;

pub struct OperationHandler;

impl AtHandler for OperationHandler {
    fn handle(&self, spawner: Spawner, at_command: AtCommand) -> Option<Error> {
        if at_command.is_query {
            let _ = spawner.spawn(operation_status_task());
            None
        } else {
            info!("Handling OPERATION command");
            if is_dds_available() {
                info!("Setting OPERATION: {:?}", at_command.id.as_str());
                let _ = spawner.spawn(operation_task(at_command));
                None
            } else {
                error!(
                    "DDS busy, cannot set OPERATION ({})",
                    at_command.id.as_str()
                );
                Some(Error::DdsBusy)
            }
        }
    }
}

#[embassy_executor::task]
async fn operation_task(at_command: AtCommand) {
    info!(
        "Sending OPERATION command to DDS task: {}",
        at_command.id.as_str()
    );
    DDS_CH.send(Msg::Operation(at_command)).await;
    info!("OPERATION command sent to DDS task");
}

#[embassy_executor::task]
async fn operation_status_task() {
    info!("Requesting OPERATION status");
    AT_CH.send(Msg::GetOperationStatus).await;
}
