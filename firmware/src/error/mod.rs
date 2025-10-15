// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCommand,     // "Not an AT command"
    InvalidBase64,      // "Invalid base64 param"
    InvalidUtf8,        // "Invalid UTF-8 in param"
    InvalidSysEx,       // "Invalid SysEx"
    InvalidDataLenght,  // "Invalid data length"
    ParamCount,         // "Param count"
    ParamValue,         // "Param value"
    NotAQuery,          // "Not a query"
    UnknownCommand,     // "Unknown command"
    FirmwareUpdateFlow, // "Firmware update flow error"
    UpdateNotStarted,   // "FW update not started"
    InvalidOffset,      // "Invalid offset"
    FlashWrite,         // "Flash write error"
    LengthMismatch,     // "Length mismatch"
    CrcMismatch,        // Crc mismatch"
    CommitFailed,       // Commit failed"
}

impl Error {
    pub fn code(&self) -> &'static str {
        match self {
            // Command errors
            Error::InvalidCommand => "E001001",
            Error::InvalidBase64 => "E001002",
            Error::InvalidUtf8 => "E001003",
            Error::InvalidSysEx => "E001004",
            Error::InvalidDataLenght => "E001005",
            Error::ParamCount => "E001006",
            Error::ParamValue => "E001007",
            Error::NotAQuery => "E001008",
            Error::UnknownCommand => "E001009",
            Error::FirmwareUpdateFlow => "E001010",
            Error::UpdateNotStarted => "E001011",
            Error::InvalidOffset => "E001012",
            Error::FlashWrite => "E001013",
            Error::LengthMismatch => "E001014",
            Error::CrcMismatch => "E001015",
            Error::CommitFailed => "E001016",
        }
    }
}
