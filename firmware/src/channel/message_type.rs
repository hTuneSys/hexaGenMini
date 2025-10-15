// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use heapless::String;

use crate::error::Error;

pub enum MsgDirection {
    Input,
    Output,
}

pub enum Msg {
    AtCmd(MsgDirection, String<64>),
    Ok,
    Err(Error),
    UsbTxLine(String<64>),
    RgbWithValue(u8, u8, u8),
}
