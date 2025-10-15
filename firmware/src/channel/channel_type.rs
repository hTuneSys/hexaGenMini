// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::Channel;

use crate::channel::Msg;

pub const CAP: usize = 16;

pub static USB_CH: Channel<Cs, Msg, CAP> = Channel::new();
pub static AT_CH: Channel<Cs, Msg, CAP> = Channel::new();
pub static RGB_CH: Channel<Cs, Msg, CAP> = Channel::new();
pub static DDS_CH: Channel<Cs, Msg, CAP> = Channel::new();
