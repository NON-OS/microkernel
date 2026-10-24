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

//! What the idle heal still does, now that it no longer repaints the whole
//! screen: an idle desktop composes nothing, and a layer whose surface did
//! not attach at first is drawn whole once it attaches, whether that is
//! found by the heal or by any other frame.

use super::attach_rig::*;
use super::frame_model::Screen;
use super::heal_rig::{heal, rig};
use crate::damage::Rect;
use crate::state::visible_drop::missed;

#[test]
fn an_idle_desktop_composes_nothing() {
    let (mut screen, mut scene, mut cache, mut kernel, mut acc) = rig();
    for (owner, r) in [(7, (0, 0, 20, 20)), (8, (30, 10, 20, 20))] {
        kernel.live.insert(owner as u64);
        open(&mut scene, &mut acc, owner, r);
    }
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    heal(&scene, &mut acc);
    assert!(acc.drain().is_none(), "nothing to compose");
    assert!(screen.px == Screen::composed(&scene).px);
}

#[test]
fn a_late_surface_is_drawn_whole_by_the_heal() {
    let (mut screen, mut scene, mut cache, mut kernel, mut acc) = rig();
    open(&mut scene, &mut acc, 7, (8, 8, 24, 16));
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    assert_eq!(missed(&scene).count(), 1, "it missed");
    kernel.live.insert(7);
    heal(&scene, &mut acc);
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    assert!(screen.px == Screen::composed(&scene).px, "drawn whole");
    assert_eq!(missed(&scene).count(), 0);
}

#[test]
fn a_late_surface_is_drawn_whole_by_any_frame() {
    let (mut screen, mut scene, mut cache, mut kernel, mut acc) = rig();
    open(&mut scene, &mut acc, 7, (8, 8, 24, 16));
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    kernel.live.insert(7);
    // The cursor moving far from the window: a frame that composes elsewhere.
    acc.accumulate(Rect { x: 50, y: 40, width: 4, height: 4 });
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    assert!(screen.px == Screen::composed(&scene).px, "drawn whole, not only where damage fell");
}
