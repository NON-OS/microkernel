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

//! The Extended Feature Register's HATS, bits 11:10 (spec 48882, 3.4.6):
//! the host page table depth a unit walks. Pure. kernel_proofs::amd_efr.

/// The levels HATS allows: 4, 5 or 6. None for the reserved value.
pub const fn hats_levels(efr: u64) -> Option<u8> {
    match (efr >> 10) & 0x3 {
        0 => Some(4),
        1 => Some(5),
        2 => Some(6),
        _ => None,
    }
}

/// Whether a unit walks the `levels`-deep tables the kernel builds (Mode 4).
pub const fn walks(efr: u64, levels: u8) -> bool {
    match hats_levels(efr) {
        Some(most) => levels <= most,
        None => false,
    }
}
