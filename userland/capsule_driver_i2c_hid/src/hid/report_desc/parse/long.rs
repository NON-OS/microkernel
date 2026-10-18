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

//! The input reports too long for one read of the input register: their
//! frames never arrive whole, so no field of theirs is decoded, and a
//! layout on one of them is not taken. They cost the device nothing else.

use super::limits::MAX_INPUT_BYTES;

#[derive(Clone, Copy, Default)]
pub(super) struct Long([u64; 4]);

impl Long {
    /// Report `id` now ends at bit `end` of its body.
    pub fn note(&mut self, id: u8, end: u32) {
        let fits = u32::from(MAX_INPUT_BYTES - u16::from(id != 0)) * 8;
        if end > fits {
            self.0[usize::from(id >> 6)] |= 1 << (id & 63);
        }
    }

    pub fn has(&self, id: u8) -> bool {
        self.0[usize::from(id >> 6)] & (1 << (id & 63)) != 0
    }
}
