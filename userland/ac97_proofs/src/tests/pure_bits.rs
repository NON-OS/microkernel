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

//! The 32-bit bound, the descriptor geometry and the command bits.

use crate::constants::ring::{GRANT_BYTES, LVI_LAG, PERIOD_SAMPLES, RING_OFFSET};
use crate::engine::entry;
use crate::setup::claim::COMMAND_BITS;
use crate::setup::dma::fits_32;

#[test]
fn a_grant_must_end_at_or_below_4_gib() {
    assert!(fits_32(0xFFFF_7000, 0x9000));
    assert!(!fits_32(0xFFFF_8000, 0x9000));
    assert!(!fits_32(u64::MAX, 2));
}

#[test]
fn every_descriptor_names_a_whole_period_inside_the_ring() {
    const { assert!(GRANT_BYTES & 0xFFF == 0) };
    // Whole stereo frames, within the descriptor's sixteen-bit count.
    const { assert!(PERIOD_SAMPLES <= 0xFFFE && PERIOD_SAMPLES & 1 == 0) };
    for i in 0..32 {
        let (addr, _) = entry(i, 0x0010_0000 + RING_OFFSET as u32);
        let end = addr as u64 + PERIOD_SAMPLES as u64 * 2;
        assert!(end <= 0x0010_0000 + GRANT_BYTES, "entry {i}");
    }
    const { assert!(LVI_LAG == 31) };
}

#[test]
fn the_command_bits_are_bus_master_and_the_silenced_pin_only() {
    assert_eq!(COMMAND_BITS, (1 << 2) | (1 << 10));
}
