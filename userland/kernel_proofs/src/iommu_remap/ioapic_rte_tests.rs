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

//! The I/O APIC's remappable redirection entry (L44b).

use super::ioapic_rte::{remappable_rte, rte_index, Line};

const LEVEL_LOW: Line = Line { level: true, active_low: true, masked: false };

#[test]
fn every_index_round_trips_and_names_no_cpu() {
    for index in (0..=u16::MAX).step_by(7).chain([0x7FFF, 0x8000, u16::MAX]) {
        let e = remappable_rte(index, 0x61, LEVEL_LOW);
        assert_eq!(rte_index(e), Some(index), "index {index:#x}");
        // Bits 47:17 are reserved in this format; a destination there would
        // be a compatibility entry's.
        assert_eq!(e & 0x0000_FFFF_FFFE_0000, 0, "index {index:#x}");
        assert_eq!(e & 0x700, 0, "delivery mode bits stay zero");
    }
}

#[test]
fn trigger_polarity_mask_and_vector_land_where_the_entry_keeps_them() {
    let edge_high = Line { level: false, active_low: false, masked: true };
    let e = remappable_rte(5, 0x42, edge_high);
    assert_eq!(e & 0xFF, 0x42);
    assert_eq!((e >> 13) & 1, 0);
    assert_eq!((e >> 15) & 1, 0);
    assert_eq!((e >> 16) & 1, 1);
    let e = remappable_rte(5, 0x42, LEVEL_LOW);
    assert_eq!(((e >> 13) & 1, (e >> 15) & 1, (e >> 16) & 1), (1, 1, 0));
    assert_eq!(rte_index(0x0000_0000_0001_A061), None, "compatibility format");
}
