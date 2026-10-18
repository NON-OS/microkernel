// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A modern common configuration in host memory with a register file per
//! queue, as a device has one: QUEUE_SIZE through QUEUE_DEVICE answer for
//! the queue QUEUE_SELECT names, and a reset leaves them as they were. A
//! plain window shows only the queue programmed last.

use nonos_virtio::common::{CommonCfg, QUEUE_DEVICE, QUEUE_SELECT, QUEUE_SIZE};
use nonos_virtio::Mmio;

use super::mem::Mem;

/// The registers a queue has its own of.
const PER_QUEUE: std::ops::Range<usize> = QUEUE_SIZE..QUEUE_DEVICE + 8;

pub struct Banked {
    pub common: Mem,
    pub queues: [Mem; 2],
}

impl Banked {
    /// Queues offering `max` entries each, the rest of the window zero.
    pub fn sized(max: [u16; 2]) -> Self {
        let queues = max.map(|m| {
            let bank = Mem::new(0x40);
            bank.put(QUEUE_SIZE, &m.to_le_bytes());
            bank
        });
        Self { common: Mem::new(0x40), queues }
    }

    /// The window a register at `off` is in for the queue selected now.
    fn at(&self, off: usize) -> Mmio {
        let selected = self.queues.get(usize::from(self.common.u16_at(QUEUE_SELECT)));
        match selected {
            Some(bank) if PER_QUEUE.contains(&off) => bank.view(),
            _ => self.common.view(),
        }
    }
}

impl CommonCfg for Banked {
    fn r8(&self, off: usize) -> u8 {
        self.at(off).r8(off)
    }
    fn r16(&self, off: usize) -> u16 {
        self.at(off).r16(off)
    }
    fn r32(&self, off: usize) -> u32 {
        self.at(off).r32(off)
    }
    fn w8(&self, off: usize, value: u8) {
        self.at(off).w8(off, value)
    }
    fn w16(&self, off: usize, value: u16) {
        self.at(off).w16(off, value)
    }
    fn w32(&self, off: usize, value: u32) {
        self.at(off).w32(off, value)
    }
}
