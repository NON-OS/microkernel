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

//! How much of the machine the RAM volume may take.

use super::PER_LEAF;

/// What the rest of the machine keeps: the larger of 1 GiB and a quarter
/// of its memory. The volume grows only while more than this is free.
pub fn reserve() -> u64 {
    (crate::memory::phys::total_memory() / 4).max(1 << 30)
}

/// Whether the volume may take more memory now. Asked once per 2 MiB of new
/// blocks, since counting what is free walks the whole frame map.
pub fn room() -> bool {
    crate::memory::phys::free_memory() > reserve()
}

/// Sectors per check of `room`.
pub const ROOM_EVERY: u64 = PER_LEAF;
