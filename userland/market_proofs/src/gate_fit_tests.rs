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
//! The Marketplace's detail pane draws only the gate rows that fit above
//! its foot (capsule_app_store store/ui/gate_fit.rs): every row in order
//! when they fit; else the failing ones first, and a line for the rest.

#[path = "../../capsule_app_store/src/store/ui/gate_fit.rs"]
mod gate_fit;
use gate_fit::shown;

#[test]
fn every_row_in_order_when_they_fit() {
    let passes = [true, false, true];
    assert_eq!(shown(&passes, 3), (vec![0, 1, 2], 0));
    assert_eq!(shown(&passes, 9), (vec![0, 1, 2], 0));
}

#[test]
fn a_short_pane_keeps_a_row_for_the_rest() {
    let passes = [true; 11];
    let (rows, more) = shown(&passes, 8);
    assert_eq!((rows.len(), more), (7, 4), "seven rows and the line for four more");
    assert_eq!(rows, (0..7).collect::<Vec<_>>(), "passing rows keep their order");
}

#[test]
fn a_failing_gate_is_never_the_one_left_out() {
    let mut passes = [true; 11];
    passes[9] = false;
    passes[10] = false;
    let (rows, more) = shown(&passes, 4);
    assert_eq!((&rows[..2], rows.len(), more), (&[9, 10][..], 3, 8));
    for room in 0..12 {
        let (rows, _) = shown(&passes, room);
        let fails = rows.iter().filter(|&&i| !passes[i]).count();
        assert!(fails == 2 || fails == rows.len(), "room {room}: {rows:?}");
    }
}

#[test]
fn no_room_draws_nothing_and_counts_everything() {
    assert_eq!(shown(&[true, false], 0), (vec![], 2));
    assert_eq!(shown(&[true, false], 1), (vec![], 2), "the one row is the line");
}
