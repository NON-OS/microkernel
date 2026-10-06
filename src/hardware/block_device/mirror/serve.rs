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

//! Reads and the disk's size, answered from the copy.

use super::find::mirror;

/// The boot disk's size in 512-byte sectors, when the loader copied from it.
pub(in crate::hardware::block_device) fn capacity() -> Option<u64> {
    mirror().map(|m| m.capacity)
}

/// Fill `out` with sectors from `lba` when one copied range holds them all;
/// false when none does, and the disk is asked instead.
pub(in crate::hardware::block_device) fn read(lba: u64, out: &mut [u8]) -> bool {
    let Some(m) = mirror() else { return false };
    let sectors = (out.len() as u64).div_ceil(512);
    let Some(end) = lba.checked_add(sectors) else { return false };
    let held = m.extents[..m.count].iter().find(|(e, _)| lba >= e.lba && end <= e.lba + e.sectors);
    let Some((e, virt)) = held else { return false };
    let from = (lba - e.lba) as usize * 512;
    // SAFETY: eK@nonos.systems - the range lies inside an extent the loader
    // filled, checked under the directmap when the record was found, and
    // nothing writes to it after the loader.
    let bytes =
        unsafe { core::slice::from_raw_parts((*virt as usize + from) as *const u8, out.len()) };
    out.copy_from_slice(bytes);
    true
}
