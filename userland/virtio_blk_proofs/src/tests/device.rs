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

//! The device's side of the rig's ring: what the driver posted, and the
//! used elements the device writes back, in the order a device writes them.

use super::model::{Rig, SIZE};
use crate::constants::STATUS_OFFSET;
use crate::queue::{avail_offset, used_offset};

impl Rig {
    /// The available index the driver published.
    pub fn posted(&self) -> u16 {
        self.ring.wrote16(avail_offset(SIZE) + 2)
    }

    /// The device writes status `s`, then uses entry `idx - 1` naming
    /// descriptor `id` and `len` bytes, then moves its index to `idx`.
    pub fn used(&self, idx: u16, id: u32, len: u32, s: u8) {
        self.header.present8(STATUS_OFFSET, s);
        let at = used_offset(SIZE) + 4 + 8 * (idx.wrapping_sub(1) % SIZE) as usize;
        self.ring.present32(at, id);
        self.ring.present32(at + 4, len);
        self.ring.present16(used_offset(SIZE) + 2, idx);
    }

    /// The request just posted, answered whole and OK, as `len` bytes.
    pub fn answer(&self, len: u32) {
        self.used(self.posted(), 0, len, 0);
    }
}
