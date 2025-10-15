// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as Cs;
use embassy_sync::channel::{Channel, Receiver, Sender};
use heapless::LinearMap;

use crate::channel::ModuleId;
use crate::channel::Msg;

pub struct ChannelManager<const CAP: usize, const MAX_CH: usize> {
    senders: LinearMap<ModuleId, Sender<'static, Cs, Msg, CAP>, MAX_CH>,
}

impl<const CAP: usize, const MAX_CH: usize> ChannelManager<CAP, MAX_CH> {
    pub const fn new() -> Self {
        Self {
            senders: LinearMap::new(),
        }
    }

    pub fn register(
        &mut self,
        id: ModuleId,
        ch: &'static Channel<Cs, Msg, CAP>,
    ) -> Receiver<'static, Cs, Msg, CAP> {
        let tx = ch.sender();
        let rx = ch.receiver();
        let _ = self.senders.insert(id, tx);
        rx
    }

    pub fn tx(&self, id: ModuleId) -> Option<Sender<'static, Cs, Msg, CAP>> {
        self.senders.get(&id).copied()
    }
}
