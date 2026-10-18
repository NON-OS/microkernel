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

//! The descriptor chain one request is, always from descriptor 0, the one
//! head a used element may name: the header the device reads, the data
//! (the device writes it for a read and reads it for a write; a flush has
//! none), and the status byte the device writes.

use super::direction::Direction;
use super::layout::Queue;
use crate::constants::{
    HEADER_OFFSET, STATUS_OFFSET, VQ_DESC_OFFSET, VRING_DESC_F_NEXT, VRING_DESC_F_WRITE,
};

const DESC_BYTES: usize = 16;
/// struct virtio_blk_req's header: type, reserved, sector.
const HEADER_LEN: u32 = 16;
/// The head every request is posted at.
pub(super) const HEAD: u16 = 0;

impl Queue {
    pub(super) fn write_chain(&self, dir: Direction, data_len: u32) {
        let header = self.header.phys.saturating_add(HEADER_OFFSET as u64);
        let status = self.header.phys.saturating_add(STATUS_OFFSET as u64);
        self.desc(0, header, HEADER_LEN, VRING_DESC_F_NEXT, 1);
        let flags = match dir {
            Direction::Flush => return self.desc(1, status, 1, VRING_DESC_F_WRITE, 0),
            Direction::Read => VRING_DESC_F_NEXT | VRING_DESC_F_WRITE,
            Direction::Write => VRING_DESC_F_NEXT,
        };
        self.desc(1, self.data.phys, data_len, flags, 2);
        self.desc(2, status, 1, VRING_DESC_F_WRITE, 0);
    }

    fn desc(&self, index: usize, addr: u64, len: u32, flags: u16, next: u16) {
        let at = VQ_DESC_OFFSET + DESC_BYTES * index;
        self.ring.w32(at, addr as u32);
        self.ring.w32(at + 4, (addr >> 32) as u32);
        self.ring.w32(at + 8, len);
        self.ring.w16(at + 12, flags);
        self.ring.w16(at + 14, next);
    }
}
