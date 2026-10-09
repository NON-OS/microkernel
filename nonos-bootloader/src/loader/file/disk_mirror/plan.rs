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

//! The live plan's imports, read as the kernel's `parse_plan` reads them.

use alloc::vec::Vec;

/* The plan's sector, in 512-byte sectors, and its first eight bytes. */
pub(super) const PLAN_LBA: u64 = 245_760;
const MAGIC: &[u8; 8] = b"NONOSDP1";
/* The kernel refuses an import below this sector, and more than 30 of them. */
const DATA_FLOOR: u64 = 262_144;
const MAX_IMPORTS: u64 = 30;

/// Each import the live plan in `sector` names, as (sector, bytes), on a
/// disk of `capacity` sectors. `None` for no plan, a plan that keeps a
/// volume on the disk (an installed NONOS, read by the kernel itself), no
/// imports, or any range the kernel would refuse.
pub(super) fn live_imports(sector: &[u8], capacity: u64) -> Option<Vec<(u64, u64)>> {
    let word = |at: usize| u64::from_le_bytes(sector[at..at + 8].try_into().unwrap_or([0; 8]));
    if sector.get(..8) != Some(&MAGIC[..]) || word(8) != 0 || word(16) != 0 {
        return None;
    }
    let count = word(24);
    if count == 0 || count > MAX_IMPORTS {
        return None;
    }
    let mut out = Vec::new();
    for i in 0..count as usize {
        let (at, bytes) = (word(32 + i * 16), word(40 + i * 16));
        let end = at.checked_add(bytes.div_ceil(512))?;
        if bytes == 0 || at < DATA_FLOOR || end > capacity {
            return None;
        }
        out.push((at, bytes));
    }
    Some(out)
}
