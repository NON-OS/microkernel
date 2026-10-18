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

//! A window reaches exactly its own buses, at its own base, and nothing it
//! was not asked for: an out-of-range field is refused, never masked onto a
//! neighbouring function.

use super::window::EcamWindow;

const VA: u64 = 0xFFFF_9000_0000_0000;
const W: EcamWindow = EcamWindow { segment: 0, start_bus: 0x10, end_bus: 0x1F, va: VA };

#[test]
fn va_maps_the_first_bus_not_bus_zero() {
    assert_eq!(W.locate(0x10, 0, 0, 0), Some(VA));
    assert_eq!(W.locate(0x11, 0, 0, 0), Some(VA + (1 << 20)));
    assert_eq!(
        W.locate(0x1F, 31, 7, 0xFFC),
        Some(VA + (0xF << 20) + (31 << 15) + (7 << 12) + 0xFFC)
    );
}

#[test]
fn a_bus_outside_the_window_is_refused() {
    assert_eq!(W.locate(0x0F, 0, 0, 0), None);
    assert_eq!(W.locate(0x20, 0, 0, 0), None);
    assert!(!W.covers(0, 0x0F) && !W.covers(0, 0x20));
    assert!(W.covers(0, 0x10) && W.covers(0, 0x1F));
    assert!(!W.covers(1, 0x10));
}

#[test]
fn out_of_range_fields_are_refused_not_masked() {
    assert_eq!(W.locate(0x10, 32, 0, 0), None);
    assert_eq!(W.locate(0x10, 0, 8, 0), None);
    assert_eq!(W.locate(0x10, 0, 0, 0x1000), None);
    assert_eq!(W.locate(0x10, 0, 0, 0xFFF), Some(VA + 0xFFF));
}

#[test]
fn bytes_is_a_megabyte_a_bus() {
    assert_eq!(W.bytes(), 16 << 20);
    let all = EcamWindow { segment: 0, start_bus: 0, end_bus: 255, va: VA };
    assert_eq!(all.bytes(), 256 << 20);
    assert_eq!(all.locate(255, 31, 7, 0xFFF), Some(VA + (256 << 20) - 1));
}
