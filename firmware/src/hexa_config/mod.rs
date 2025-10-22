// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use core::sync::atomic::{AtomicBool, Ordering};

//General configuration constants
pub const CONF_VERSION: &str = "v1.0.0";

//DDS status tracking
pub static DDS_AVAILABLE: AtomicBool = AtomicBool::new(true);
pub fn set_dds_available(status: bool) {
    DDS_AVAILABLE.store(status, Ordering::SeqCst);
}
pub fn is_dds_available() -> bool {
    DDS_AVAILABLE.load(Ordering::SeqCst)
}
