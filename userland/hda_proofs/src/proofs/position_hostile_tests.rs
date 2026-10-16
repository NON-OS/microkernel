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

//! LPIB and the position buffer are whatever the controller writes, and the
//! refill indexes the PCM ring by the position it is handed. Every reading,
//! from either source, must come back inside the ring.

use crate::controller::bdl::{N_PERIODS, PERIOD_BYTES, RING_BYTES};
use crate::controller::position::Position;

const HOSTILE: [u32; 8] = [
    0,
    1,
    RING_BYTES as u32 - 1,
    RING_BYTES as u32,
    RING_BYTES as u32 + 1,
    0x7fff_ffff,
    0xffff_fffe,
    u32::MAX,
];

#[test]
fn every_reading_from_either_source_lands_inside_the_ring() {
    for buffer in [true, false] {
        for &b in &HOSTILE {
            for &l in &HOSTILE {
                let pos = Position::new(buffer).pick(b, l, RING_BYTES as u32);
                assert!((pos as u64) < RING_BYTES, "{b:#x}/{l:#x} gave {pos:#x}");
                assert!(((pos as u64 / PERIOD_BYTES) as usize) < N_PERIODS);
            }
        }
    }
}

#[test]
fn a_ring_of_no_bytes_does_not_divide_by_zero() {
    assert_eq!(Position::new(false).pick(0, u32::MAX, 0), u32::MAX);
}
