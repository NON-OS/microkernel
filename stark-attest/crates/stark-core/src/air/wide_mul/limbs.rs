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

pub const LIMB_BITS: u32 = 16;
pub const LIMB_MASK: u64 = (1u64 << LIMB_BITS) - 1;
pub const N_LIMBS: usize = 4;

/// A 64 bit value as four 16 bit limbs, little endian. Every limb must be range
/// checked in circuit: a decomposition whose limbs are unbounded reassembles to
/// anything, which is the wraparound the gadget exists to close.
pub fn split(v: u64) -> [u64; N_LIMBS] {
    let mut l = [0u64; N_LIMBS];
    for (i, x) in l.iter_mut().enumerate() {
        *x = (v >> (LIMB_BITS * i as u32)) & LIMB_MASK;
    }
    l
}
