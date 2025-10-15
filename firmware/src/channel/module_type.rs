// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleId {
    Usb,
    At,
    Rgb,
    Dds,
}
