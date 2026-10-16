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

use crate::grab_rule::{steps, wanted, Modal, KEY_DOWN_BIT, POINTER_BITS};

fn with(f: impl FnOnce(&mut Modal)) -> u32 {
    let mut m = Modal::default();
    f(&mut m);
    wanted(m)
}

#[test]
fn every_thing_drawn_over_the_windows_takes_the_pointer_and_what_answers_keys_takes_them() {
    assert_eq!(with(|_| {}), 0, "a bare desktop holds nothing");
    assert_eq!(with(|m| m.menu = true), POINTER_BITS | KEY_DOWN_BIT, "Esc closes a menu");
    assert_eq!(with(|m| m.dialog = true), POINTER_BITS | KEY_DOWN_BIT, "Tab, Enter, Esc answer it");
    assert_eq!(with(|m| m.drag = true), POINTER_BITS);
    assert_eq!(with(|m| m.launchpad = true), POINTER_BITS | KEY_DOWN_BIT);
    assert_eq!(with(|m| m.rename = true), KEY_DOWN_BIT, "a rename edits, the pointer stays free");
}

#[test]
fn the_pointer_grab_carries_presses_releases_and_absolute_motion() {
    for kind in [3u32, 4, 5, 6, 7] {
        assert_ne!(POINTER_BITS & (1 << kind), 0, "kind {kind}");
    }
    assert_eq!(POINTER_BITS & (1 << 2), 0, "relative motion is left to the router");
    assert_eq!(POINTER_BITS & KEY_DOWN_BIT, 0);
}

#[test]
fn moving_between_holdings_releases_only_when_something_is_dropped() {
    assert_eq!(steps(0, 0), (false, None));
    assert_eq!(steps(0, POINTER_BITS), (false, Some(POINTER_BITS)));
    assert_eq!(steps(POINTER_BITS, POINTER_BITS), (false, None), "no call when nothing changes");
    assert_eq!(steps(POINTER_BITS, 0), (true, None));
    // A rename under an open menu: adding keys is one request.
    let both = POINTER_BITS | KEY_DOWN_BIT;
    assert_eq!(steps(POINTER_BITS, both), (false, Some(both)));
    // The menu closes while the rename goes on: the router drops both grabs
    // on a release, so the keys are asked for again after it.
    assert_eq!(steps(both, KEY_DOWN_BIT), (true, Some(KEY_DOWN_BIT)));
}
