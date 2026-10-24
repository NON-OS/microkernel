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

//! The `BOX` entry at its limits: the total held at its maximum, values out
//! of range, and bodies one byte short or long.

use super::record_box::{layout, run};
use nsh_wire::record::BOX_BODY;
use nsh_wire::WireError;

#[test]
fn the_total_holds_at_its_limit() {
    let (full, some) = (
        layout(0, 1, 0, [u64::MAX, 1, 0, 0, 0, 0, 0, 0, 7]),
        layout(0, 1, 0, [3, 0, 0, 0, 0, 0, 0, 0, 4]),
    );
    assert_eq!(run(&full).unwrap().refused(), u64::MAX);
    assert_eq!(run(&some).unwrap().refused(), 7);
}

#[test]
fn out_of_range_and_wrong_lengths_are_refused() {
    for (preset, started) in [(3, 1), (255, 1), (0, 2), (0, 255)] {
        let b = layout(preset, started, 0, [0; 9]);
        assert_eq!(run(&b), Err(WireError::Value), "{preset} {started}");
    }
    let good = layout(0, 1, 0, [0; 9]);
    assert_eq!(run(&good[..BOX_BODY - 1]), Err(WireError::Short));
    assert_eq!(run(&[&good[..], &[0]].concat()), Err(WireError::Trailing));
}
