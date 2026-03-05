// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use crate::AT_CH;
use crate::at::encode_response;
use crate::channel::*;
use crate::hexa_config::CONF_VERSION;

#[embassy_executor::task]
pub async fn version_task() {
    let line = encode_response(b"VERSION", 0, &[CONF_VERSION.as_bytes()]);
    AT_CH.send(Msg::AtCmdResponse(line)).await;
}
