// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::info;

use hexa_tune_proto_embedded::command::OperationSub;

use crate::AT_CH;
use crate::DDS_CH;
use crate::channel::*;

#[embassy_executor::task]
pub async fn operation_task(id: u32, sub: OperationSub) {
    info!("Sending OPERATION command to DDS task");
    DDS_CH.send(Msg::OperationCmd { id, sub }).await;
    info!("OPERATION command sent to DDS task");
}

#[embassy_executor::task]
pub async fn operation_status_task() {
    info!("Requesting OPERATION status");
    AT_CH.send(Msg::GetOperationStatus).await;
}
