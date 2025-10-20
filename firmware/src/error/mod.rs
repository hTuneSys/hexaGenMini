// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCommand,    // "Not an AT command"
    DdsBusy,           // "DDS is busy"
    InvalidUtf8,       // "Invalid UTF-8 in param"
    InvalidSysEx,      // "Invalid SysEx"
    InvalidDataLenght, // "Invalid data length"
    ParamCount,        // "Param count"
    ParamValue,        // "Param value"
    NotAQuery,         // "Not a query"
    UnknownCommand,    // "Unknown command"
}

impl Error {
    pub fn code(&self) -> &'static str {
        match self {
            // Command errors
            Error::InvalidCommand => "E001001",
            Error::DdsBusy => "E001002",
            Error::InvalidUtf8 => "E001003",
            Error::InvalidSysEx => "E001004",
            Error::InvalidDataLenght => "E001005",
            Error::ParamCount => "E001006",
            Error::ParamValue => "E001007",
            Error::NotAQuery => "E001008",
            Error::UnknownCommand => "E001009",
        }
    }
}
