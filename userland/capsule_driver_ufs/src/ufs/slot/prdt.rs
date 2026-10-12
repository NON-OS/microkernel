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

//! The PRDT entry for the data region (layout in facts::utp).

use crate::ufs::facts::utp::{PRDT_ENTRY, PRDT_MAX_BYTES};

/// The entry for `len` bytes at `bus`; None for a length the field cannot
/// carry (empty, not whole dwords, past 256 KiB) or an unaligned address,
/// which the host would refuse.
pub fn entry(bus: u64, len: usize) -> Option<[u8; PRDT_ENTRY]> {
    let fits = (4..=PRDT_MAX_BYTES).contains(&len) && len.is_multiple_of(4);
    if !fits || !bus.is_multiple_of(4) {
        return None;
    }
    let mut out = [0u8; PRDT_ENTRY];
    out[0..8].copy_from_slice(&bus.to_le_bytes());
    out[12..16].copy_from_slice(&((len - 1) as u32).to_le_bytes());
    Some(out)
}
