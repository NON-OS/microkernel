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

//! The edges of a window: empty ones reach nothing, overlap is decided per
//! segment with both ends counted, and no address wraps past the top.

use super::window::EcamWindow;

const VA: u64 = 0xFFFF_9000_0000_0000;
const W: EcamWindow = EcamWindow { segment: 0, start_bus: 0x10, end_bus: 0x1F, va: VA };

#[test]
fn an_empty_window_reaches_nothing() {
    let backwards = EcamWindow { segment: 0, start_bus: 5, end_bus: 4, va: VA };
    let unmapped = EcamWindow { va: 0, ..W };
    for w in [backwards, unmapped] {
        assert!(w.is_empty());
        assert_eq!(w.bytes(), 0);
        assert_eq!(w.locate(5, 0, 0, 0), None);
    }
}

#[test]
fn overlap_is_per_segment_and_inclusive() {
    let next = EcamWindow { start_bus: 0x20, end_bus: 0x2F, ..W };
    let touching = EcamWindow { start_bus: 0x1F, end_bus: 0x2F, ..W };
    let other_segment = EcamWindow { segment: 1, ..W };
    assert!(!W.overlaps(&next) && !next.overlaps(&W));
    assert!(W.overlaps(&touching) && touching.overlaps(&W));
    assert!(!W.overlaps(&other_segment));
}

#[test]
fn an_address_past_the_top_of_memory_is_refused() {
    let high = EcamWindow { va: u64::MAX - 0xFFF, ..W };
    assert_eq!(high.locate(0x10, 0, 0, 0xFFF), Some(u64::MAX));
    assert_eq!(high.locate(0x11, 0, 0, 0), None);
}
