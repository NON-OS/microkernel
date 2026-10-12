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

//! The used ring, which the device writes: every value read here is the
//! device's word and is checked by whoever uses it.

use nonos_virtio::Mmio;

use crate::constants::{RING_SLOTS, VQ_USED_OFFSET};

const USED_IDX: usize = VQ_USED_OFFSET + 2;
const USED_RING: usize = VQ_USED_OFFSET + 4;
const USED_ELEM_SIZE: usize = 8;

/// How many entries the device has used, ever (it wraps).
pub fn idx(ring: Mmio) -> u16 {
    ring.r16(USED_IDX)
}

/// The descriptor id and used length of entry `n`.
pub fn elem(ring: Mmio, n: u16) -> (u32, u32) {
    let at = USED_RING + USED_ELEM_SIZE * usize::from(n % RING_SLOTS);
    (ring.r32(at), ring.r32(at + 4))
}
