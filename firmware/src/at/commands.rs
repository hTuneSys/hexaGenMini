// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use crate::at::handler::{SumHandler, VersionHandler};
use heapless::{LinearMap, String};

pub enum Handler {
    Version(VersionHandler),
    Sum(SumHandler),
}

pub fn register_all() -> LinearMap<String<16>, Handler, 8> {
    let mut map: LinearMap<String<16>, Handler, 8> = LinearMap::new();
    let mut version_key = String::<16>::new();
    version_key.push_str("VERSION").unwrap();
    map.insert(version_key, Handler::Version(VersionHandler))
        .ok();
    let mut sum_key = String::<16>::new();
    sum_key.push_str("SUM").unwrap();
    map.insert(sum_key, Handler::Sum(SumHandler)).ok();
    map
}
