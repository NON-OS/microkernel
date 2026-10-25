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

//! Whether a function has come back, and the marker that shows a reset
//! reached it (L78). Only the Vendor ID half of dword 0 decides: all ones
//! (nothing answered, or the read failed) and the retry status say not yet,
//! every other vendor says back, whatever the Device ID half holds. The
//! marker is INTx Disable where it sticks, else Memory Space, else none; each
//! is one Command bit the put-back rewrites, and never Bus Master.

use super::bits_rule::{not_ready, BME, INTX_DISABLE, IO, MEM};
use super::marker_rule::{marker, marker_bit, Marker};

#[test]
fn only_all_ones_or_the_retry_status_in_the_vendor_half_is_not_ready() {
    // A Device ID half equal to either value must not count (recalled, unverified).
    let devices = [0x0000u32, 0x0001, 0x10D3, 0x8086, 0xFFFF];
    let mut not = 0u32;
    for vendor in 0..=0xFFFFu32 {
        for device in devices {
            let id = vendor | device << 16;
            let want = vendor == 0xFFFF || vendor == 0x0001;
            assert_eq!(not_ready(id), want, "{id:#010x}");
            not += u32::from(want);
        }
    }
    assert_eq!(not, 10, "two vendors under each of five devices");
    assert!(not_ready(u32::MAX), "a failed read is never taken for an answer");
    assert!(!not_ready(0xFFFF_8086) && !not_ready(0x0001_1AF4), "a real vendor is back");
}

#[test]
fn the_marker_is_intx_disable_first_then_memory_space_else_none() {
    let table = [
        ((false, false), None),
        ((false, true), Some(Marker::MemorySpace)),
        ((true, false), Some(Marker::IntxDisable)),
        ((true, true), Some(Marker::IntxDisable)),
    ];
    for ((intx, mem), want) in table {
        assert_eq!(marker(intx, mem), want, "INTx Disable back {intx}, Memory Space back {mem}");
    }
    // Command bits 10 and 1 (recalled, unverified).
    assert_eq!(marker_bit(Marker::IntxDisable), 1 << 10);
    assert_eq!(marker_bit(Marker::MemorySpace), 1 << 1);
}

/// The put-back clears IO, Memory, Bus Master and INTx Disable and gives the
/// decoders back from what was saved, so a marker bit set for the wipe never
/// reaches the next holder; Bus Master as a marker would let the function
/// master through the window.
#[test]
fn a_marker_is_one_command_bit_the_put_back_rewrites_never_bus_master() {
    let rewritten = IO | MEM | BME | INTX_DISABLE;
    let bits = [Marker::IntxDisable, Marker::MemorySpace].map(marker_bit);
    for b in bits {
        assert_eq!(b.count_ones(), 1, "{b:#06x}");
        assert_eq!(b & rewritten, b, "{b:#06x} outlives the put-back");
        assert_eq!(b & BME, 0, "{b:#06x}");
    }
    assert_ne!(bits[0], bits[1], "two markers on one bit");
}
