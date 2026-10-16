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

//! F-100: the processors' thermal limits beside their _PPC. A limit is a
//! percentage, never past 100; 100 lifts it; the same limit again changes
//! nothing; the list never grows past its cap; and a processor with no
//! entry runs unlimited.

use super::acpi::thermal_limit_rule::{ceiling, place, Placed, FULL};

#[test]
fn a_limit_is_set_moved_and_lifted() {
    let mut l = Vec::new();
    assert_eq!(ceiling(&l, 3), 100, "no entry: unlimited");
    assert_eq!(place(&mut l, 3, 60, 4), Placed::Set);
    assert_eq!(ceiling(&l, 3), 60);
    assert_eq!(place(&mut l, 3, 60, 4), Placed::Same);
    assert_eq!(place(&mut l, 3, 0, 4), Placed::Set);
    assert_eq!(ceiling(&l, 3), 0);
    assert_eq!(place(&mut l, 3, FULL, 4), Placed::Cleared);
    assert!(l.is_empty(), "lifting a limit leaves no entry behind");
    assert_eq!(place(&mut l, 3, FULL, 4), Placed::Same);
    assert_eq!(ceiling(&l, 3), 100);
}

#[test]
fn past_100_is_refused_and_nothing_changes() {
    let mut l = vec![(1, 40)];
    for pct in [101, 255, 256, u64::MAX] {
        assert_eq!(place(&mut l, 1, pct, 4), Placed::Bad, "{pct}");
        assert_eq!(place(&mut l, 2, pct, 4), Placed::Bad, "{pct}");
    }
    assert_eq!(l, [(1, 40)]);
}

#[test]
fn the_list_stops_at_its_cap() {
    let mut l = Vec::new();
    for uid in 0..4 {
        assert_eq!(place(&mut l, uid, 50, 4), Placed::Set);
    }
    assert_eq!(place(&mut l, 9, 50, 4), Placed::Full);
    assert_eq!(l.len(), 4);
    assert_eq!(place(&mut l, 2, 30, 4), Placed::Set, "a limited processor still moves");
    assert_eq!(place(&mut l, 9, FULL, 4), Placed::Same, "lifting an absent limit takes no slot");
    assert_eq!(ceiling(&l, 9), 100);
}
