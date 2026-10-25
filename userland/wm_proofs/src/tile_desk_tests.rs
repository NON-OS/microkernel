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
//! Workspaces: a window opens on the one shown; switching says what to hide
//! and what to show; a window sent on keeps floating if it floated; focus
//! passes on when its window goes.

use crate::tile::desk::{Desk, SPACES};

const A: (u32, u32) = (1, 1);
const B: (u32, u32) = (2, 1);
const C: (u32, u32) = (3, 1);

#[test]
fn switching_hides_one_workspace_and_shows_another() {
    let mut d = Desk::new();
    d.open(A);
    d.open(B);
    assert_eq!(d.switch(2), Some((vec![A, B], vec![])));
    d.open(C);
    assert_eq!(d.switch(0), Some((vec![C], vec![A, B])));
    assert_eq!(d.switch(0), None, "already shown");
    assert_eq!(d.switch(SPACES), None, "no such workspace");
    assert_eq!(d.shown, 0);
}

#[test]
fn a_window_sent_on_keeps_floating() {
    let mut d = Desk::new();
    d.open(A);
    d.open(B);
    assert!(d.spaces[0].toggle_float(B));
    assert!(d.send(B, 4));
    assert_eq!(d.spaces[4].floating, [B]);
    assert_eq!(d.spaces[0].windows(), [A]);
    assert!(d.send(A, 4));
    assert_eq!(d.spaces[4].windows(), [A, B], "tiled first, then floating");
    assert!(!d.send(A, 4), "already there");
    assert!(!d.send(A, SPACES) && !d.send(C, 1), "no such workspace or window");
}

#[test]
fn focus_passes_on_and_floating_toggles_back() {
    let mut d = Desk::new();
    d.open(A);
    d.open(B);
    assert_eq!(d.spaces[0].focus, Some(B));
    assert!(d.spaces[0].toggle_float(A));
    assert_eq!((d.spaces[0].focus, d.spaces[0].floating.clone()), (Some(A), vec![A]));
    assert!(d.spaces[0].toggle_float(A), "tiled again");
    assert!(d.spaces[0].floating.is_empty() && d.spaces[0].tiled(A));
    d.close(A);
    assert_eq!(d.spaces[0].focus, Some(B), "the first window left");
    d.close(B);
    assert_eq!((d.spaces[0].focus, d.spaces[0].windows()), (None, vec![]));
}
