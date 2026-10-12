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

//! The request ring in host memory, and the device's side of it: it fills
//! the buffer descriptor 0 names with a byte of its choosing, then writes a
//! used element and, after it, the index that covers it.

use nonos_devmodel::FakeBar;

use super::model::view;
use crate::constants::{ENTROPY_BUF_LEN, VQ_REGION_SIZE, VQ_USED_OFFSET};
use crate::queue::Queue;

/// QEMU's virtio-rng offers an 8-entry queue.
pub const SIZE: u16 = 8;
pub const BUF_PHYS: u64 = 0x7700_0000;

pub struct Ring {
    pub region: FakeBar,
    pub buf: FakeBar,
}

impl Ring {
    /// Fresh grants: the broker hands them over zeroed.
    pub fn new() -> Self {
        Self { region: FakeBar::new(VQ_REGION_SIZE), buf: FakeBar::new(ENTROPY_BUF_LEN) }
    }

    pub fn queue(&self) -> Queue {
        Queue::new(view(&self.region), view(&self.buf), BUF_PHYS, SIZE)
    }

    /// The available ring's index, or with `Some(pos)` its entry there.
    pub fn avail(&self, pos: Option<usize>) -> u16 {
        self.region.wrote16(SIZE as usize * 16 + pos.map_or(2, |p| 4 + 2 * p))
    }

    pub fn used_idx(&self) -> u16 {
        self.region.wrote16(VQ_USED_OFFSET + 2)
    }

    /// The device fills the buffer with `byte`, then uses ring entry
    /// `used_idx - 1`, naming descriptor `id` and `len` bytes.
    pub fn complete(&self, used_idx: u16, id: u32, len: u32, byte: u8) {
        (0..self.buf.len()).for_each(|at| self.buf.present8(at, byte));
        self.used(used_idx, id, len);
    }

    /// A used element and its index, with nothing written to the buffer.
    pub fn used(&self, used_idx: u16, id: u32, len: u32) {
        let elem = VQ_USED_OFFSET + 4 + 8 * (used_idx.wrapping_sub(1) % SIZE) as usize;
        self.region.present32(elem, id);
        self.region.present32(elem + 4, len);
        self.region.present16(VQ_USED_OFFSET + 2, used_idx);
    }

    /// The device answers the next request it has not answered, whole.
    pub fn answer(&self, byte: u8) {
        self.complete(self.used_idx().wrapping_add(1), 0, ENTROPY_BUF_LEN as u32, byte);
    }
}
