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

//! HPET from the table's bytes, with no pointer read: after the 36-byte
//! header, the event timer block id u32 at 36, then the base address as a
//! Generic Address Structure at 40 (space id u8, bit width, bit offset,
//! access size, then the address u64 at 44), the HPET number at 52 and the
//! minimum tick u16 at 53 (ACPI's HPET table, recalled: unverified). The
//! block is memory-mapped, so any other address space is refused, as is a
//! table whose declared or handed length stops before the address ends.

pub(crate) const HPET_LEN: usize = 56;
const SPACE_MEMORY: u8 = 0;

/// The block's physical base, or None for a short table, a block that is
/// not in memory, or no address.
pub(crate) fn hpet_base(t: &[u8]) -> Option<u64> {
    let declared = u32::from_le_bytes(t.get(4..8)?.try_into().ok()?) as usize;
    let t = t.get(..declared.min(t.len())).filter(|t| t.len() >= HPET_LEN)?;
    let base = u64::from_le_bytes(t[44..52].try_into().ok()?);
    (t[40] == SPACE_MEMORY && base != 0).then_some(base)
}
