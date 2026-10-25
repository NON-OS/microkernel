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

//! What firmware's memory holds where the bootloader says the SMBIOS entry
//! point is, for F-105b: an F-105 entry point, honest or lying, and then
//! whatever bytes follow it.

use super::seeded_entries::{fix, honest, lie};
use super::seeded_gen::Rng;
use crate::kernel::smbios_entry::TABLE_MAX;

/// Most 64-bit seeds get a maximum size near `TABLE_MAX`, either side, so
/// the kernel's own bound is met as often as it is passed; some a table so
/// near the top of the address space that its end would wrap.
fn believable(r: &mut Rng, b: &mut [u8]) {
    if r.below(4) != 0 {
        let max = r.below(2 * u64::from(TABLE_MAX)) as u32;
        b[12..16].copy_from_slice(&max.to_le_bytes());
    }
    if r.below(6) == 0 {
        b[16..24].copy_from_slice(&(u64::MAX - r.below(u64::from(TABLE_MAX))).to_le_bytes());
    }
    fix(b, 0..0x18, 5);
}
/// Whether the entry point is the 64-bit form, and the memory from it on.
pub fn firmware(r: &mut Rng) -> (bool, Vec<u8>) {
    let wide = r.below(2) == 0;
    let mut b = honest(r, wide);
    if wide {
        believable(r, &mut b);
    }
    lie(r, &mut b, wide);
    b.extend(r.bytes(40, 0));
    (wide, b)
}
