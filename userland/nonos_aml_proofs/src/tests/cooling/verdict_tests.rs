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

//! What a reading calls for: trip points reached, active levels with
//! their hysteresis, and the conversions both ways.

use crate::thermal::{judge, milli_celsius, tenths_kelvin, Trips};

fn trips() -> Trips {
    Trips {
        critical: Some(100_000),
        passive: Some(80_000),
        active: vec![70_000, 60_000],
        ..Trips::default()
    }
}

#[test]
fn levels_start_at_their_trip_and_stop_two_degrees_below() {
    let t = trips();
    assert_eq!(judge(&t, 59_999, &[]).active, [false, false]);
    assert_eq!(judge(&t, 60_000, &[]).active, [false, true]);
    assert_eq!(judge(&t, 58_001, &[false, true]).active, [false, true]);
    assert_eq!(judge(&t, 58_000, &[false, true]).active, [false, false]);
    assert_eq!(judge(&t, 58_001, &[]).active, [false, false], "never started");
    assert_eq!(judge(&t, 75_000, &[]).active, [true, true]);
}

#[test]
fn trip_points_reached() {
    let v = judge(&trips(), 100_000, &[]);
    assert!(v.critical && v.passive && !v.hot, "no _HOT, never hot");
    let v = judge(&trips(), 79_999, &[]);
    assert!(!v.critical && !v.passive);
}

#[test]
fn kelvin_both_ways() {
    assert_eq!(milli_celsius(2732), Some(50));
    assert_eq!(tenths_kelvin(50), 2732);
    assert_eq!(tenths_kelvin(-50_050), 2231);
    assert_eq!(tenths_kelvin(i32::MIN), 0);
}
