// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use heapless::String;

use crate::error::FirmwareError;
use hexa_tune_proto_embedded::command::OperationSub;

pub type MsgId = u32;
pub type MsgString = String<64>;

pub enum Msg {
    AtRxLine(MsgString),
    AtCmdResponse(MsgString),
    Done(MsgId),
    Err(MsgId, FirmwareError),
    UsbTxLine(MsgString),
    RgbSet { id: u32, r: u8, g: u8, b: u8 },
    FreqSet { id: u32, freq: u32, time_ms: u32 },
    SetDdsAvailable(bool),
    SetOperationStatus(MsgString),
    GetOperationStatus,
    OperationCmd { id: u32, sub: OperationSub },
}
