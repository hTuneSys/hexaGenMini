// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::info;

use crate::DDS_CH;
use crate::channel::*;

#[embassy_executor::task]
pub async fn freq_task(id: u32, freq: u32, time_ms: u32) {
    info!("Sending FREQ command to DDS task");
    DDS_CH.send(Msg::FreqSet { id, freq, time_ms }).await;
    info!("FREQ command sent to DDS task");
}
