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

use crate::arch::fdt::Fdt;

/// Keep `/chosen` `rng-seed` for the RNG, and say where its bytes lie in the
/// blob, as an offset and length, for `erase` once the parser is done.
pub(crate) fn keep(fdt: &Fdt, blob: u64) -> Option<(u64, usize)> {
    let seed = fdt.chosen_bytes(b"rng-seed").ok()??;
    crate::arch::aarch64::boot_entropy::keep(seed);
    Some((seed.as_ptr() as u64 - blob, seed.len()))
}

/// Zero the seed in the blob, so nothing that reads the tree later, the
/// broker's platform pass among them, finds the bytes the RNG was keyed from.
///
/// # Safety
///
/// `blob` points at the tree `keep` read, `at` came from `keep` on it, and
/// no reference into the blob is live.
pub(crate) unsafe fn erase(blob: *mut u8, at: (u64, usize)) {
    let (offset, len) = at;
    for i in 0..len {
        // SAFETY: inside the property's value, within the blob the loader
        // left in RAM, which boot reserves and nothing else is reading.
        unsafe { blob.add(offset as usize + i).write_volatile(0) };
    }
}
