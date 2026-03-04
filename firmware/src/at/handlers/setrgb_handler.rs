// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use crate::RGB_CH;
use crate::channel::*;

#[embassy_executor::task]
pub async fn setrgb_task(id: u32, r: u8, g: u8, b: u8) {
    RGB_CH.send(Msg::RgbSet { id, r, g, b }).await;
}
