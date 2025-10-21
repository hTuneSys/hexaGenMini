// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use heapless::String;

use crate::at::AtCommand;
use crate::error::Error;

pub type MsgId = String<16>;
pub type MsgString = String<64>;
pub type IsDdsAvailable = bool;

pub enum Msg {
    AtRxLine(MsgString),
    AtCmdOutput(AtCommand),
    Done(MsgId),
    Err(MsgId, Error),
    ErrWOCommand(Error),
    UsbTxLine(MsgString),
    RgbWithValue(AtCommand),
    FreqWithValue(AtCommand),
    SetDdsAvailable(IsDdsAvailable),
}
