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
    ///RGB value message with R, G, B u8 values
    RgbWithValue(u8, u8, u8),
    ///Frequency message with ID, frequency in Hz, and time in ms
    FreqWithValue(String<16>, u32, u32),
}
