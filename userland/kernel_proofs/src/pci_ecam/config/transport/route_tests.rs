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

//! A window wins wherever it covers; without one only a PC's ports reach,
//! and only segment 0 below 0x100; everything else is unreachable.

use super::ecam::EcamWindow;
use super::route::{route, Route};

const W: EcamWindow = EcamWindow { segment: 0, start_bus: 0, end_bus: 0, va: 0x8000_0000 };

#[test]
fn a_window_wins_over_the_ports_at_any_offset() {
    for offset in [0, 0xFF, 0x100, 0xFFC] {
        assert_eq!(route(Some(W), true, 0, offset), Route::Ecam(W));
        assert_eq!(route(Some(W), false, 0, offset), Route::Ecam(W));
    }
}

#[test]
fn the_ports_reach_segment_zero_below_0x100_only() {
    assert_eq!(route(None, true, 0, 0), Route::Legacy);
    assert_eq!(route(None, true, 0, 0xFC), Route::Legacy);
    assert_eq!(route(None, true, 0, 0xFF), Route::Legacy);
    assert_eq!(route(None, true, 0, 0x100), Route::Unreachable);
    assert_eq!(route(None, true, 0, 0xFFF), Route::Unreachable);
    assert_eq!(route(None, true, 1, 0), Route::Unreachable);
    assert_eq!(route(None, true, 0x1000, 0), Route::Unreachable);
}

#[test]
fn without_ports_or_a_window_nothing_is_reachable() {
    assert_eq!(route(None, false, 0, 0), Route::Unreachable);
    assert_eq!(route(None, false, 0, 0x100), Route::Unreachable);
}
