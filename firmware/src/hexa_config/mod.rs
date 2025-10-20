// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use core::sync::atomic::{AtomicBool, Ordering};

pub const CONF_VERSION: &str = "v1.0.0";

pub static DDS_GENERATING: AtomicBool = AtomicBool::new(false);
pub fn set_dds_status(status: bool) {
    DDS_GENERATING.store(status, Ordering::SeqCst);
}
pub fn is_dds_available() -> bool {
    !DDS_GENERATING.load(Ordering::SeqCst)
}
