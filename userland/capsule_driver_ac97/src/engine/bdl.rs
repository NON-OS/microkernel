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

//! The thirty-two descriptors. Entry `i` names period `i % N_PERIODS`, so
//! the list walks the four periods eight times over and the engine, kept
//! running by the last-valid index, loops the ring for as long as it plays.

use core::ptr::write_volatile;

use crate::constants::ring::{BDL_ENTRIES, BD_BUP, N_PERIODS, PERIOD_BYTES, PERIOD_SAMPLES};

/// Descriptor `i`: the bus address of its period and its control word.
pub const fn entry(i: usize, ring_dev: u32) -> (u32, u32) {
    let offset = ((i % N_PERIODS) * PERIOD_BYTES) as u32;
    (ring_dev + offset, BD_BUP | PERIOD_SAMPLES)
}

/// Write the list at `bdl_va`, which must map `BDL_ENTRIES * 8` writable
/// bytes: the driver's DMA grant, held until the driver exits.
pub fn fill(bdl_va: u64, ring_dev: u32) {
    let words = bdl_va as *mut u32;
    for i in 0..BDL_ENTRIES {
        let (addr, ctl) = entry(i, ring_dev);
        // SAFETY: within the list's bytes of the grant, per the contract
        // above; the engine reads them only once it runs.
        unsafe {
            write_volatile(words.add(2 * i), addr.to_le());
            write_volatile(words.add(2 * i + 1), ctl.to_le());
        }
    }
}
