// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCommand, // "Not an AT command"
    InvalidBase64,  // "Invalid base64 param"
    InvalidUtf8,    // "Invalid UTF-8 in param"
    InvalidSysEx,   // "Invalid SysEx"
    ParamCount,     // "Param count"
    ParamValue,     // "Param value"
    NotAQuery,      // "Not a query"
    UnknownCommand, // "Unknown command"
}

impl Error {
    pub fn description(&self) -> &'static str {
        match self {
            Error::InvalidCommand => "Not an AT command",
            Error::InvalidBase64 => "Invalid base64 param",
            Error::InvalidUtf8 => "Invalid UTF-8 in param",
            Error::InvalidSysEx => "Invalid SysEx",
            Error::ParamCount => "Invalid param count",
            Error::ParamValue => "Invalid param value",
            Error::NotAQuery => "Not a query",
            Error::UnknownCommand => "Unknown command",
        }
    }
}
