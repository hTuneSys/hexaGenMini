// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use core::fmt::Write;

/// Trait for AT command handlers
pub trait AtHandler {
    fn handle(
        &self,
        params: &[heapless::String<16>],
        is_query: bool,
    ) -> Result<heapless::String<64>, crate::error::Error>;
}

/// Example: Version query handler
pub struct VersionHandler;

impl AtHandler for VersionHandler {
    fn handle(
        &self,
        _params: &[heapless::String<16>],
        is_query: bool,
    ) -> Result<heapless::String<64>, crate::error::Error> {
        if is_query {
            let mut out = heapless::String::<64>::new();
            out.push_str("1.2.3")
                .map_err(|_| crate::error::Error::InvalidUtf8)?;
            Ok(out)
        } else {
            Err(crate::error::Error::NotAQuery)
        }
    }
}

/// Example: Sum handler (takes two params, returns their sum)
pub struct SumHandler;

impl AtHandler for SumHandler {
    fn handle(
        &self,
        params: &[heapless::String<16>],
        _is_query: bool,
    ) -> Result<heapless::String<64>, crate::error::Error> {
        if params.len() != 2 {
            return Err(crate::error::Error::ParamCount);
        }
        let a = params[0]
            .parse::<i32>()
            .map_err(|_| crate::error::Error::ParamValue)?;
        let b = params[1]
            .parse::<i32>()
            .map_err(|_| crate::error::Error::ParamValue)?;
        let mut out = heapless::String::<64>::new();
        write!(out, "{}", a + b).map_err(|_| crate::error::Error::InvalidUtf8)?;
        Ok(out)
    }
}
