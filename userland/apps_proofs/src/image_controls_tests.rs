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

//! The image viewer's controls: a click or a hover at a control's centre
//! names it, prev and next never overlap and show only in the single view with
//! another image, and a click names the points the old hit test did.

use nonos_toolkit::widgets::Rect;

use crate::viewer::target::{controls, target_at, Shown, Target};

pub const SIZES: [(u32, u32); 4] = [(720, 520), (320, 240), (1920, 1080), (96, 64)];

pub fn single(w: u32, h: u32, images: usize) -> Shown {
    Shown { gallery: false, images, w, h, scroll: 0, tiles: 0 }
}

pub fn mid(r: Rect) -> (i32, i32) {
    ((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32)
}

#[test]
fn every_control_is_named_by_the_point_at_its_centre_and_none_overlap() {
    for (w, h) in SIZES {
        let s = single(w, h, 3);
        let cs: Vec<(Target, Rect)> = controls(&s).collect();
        assert_eq!(cs.iter().map(|c| c.0).collect::<Vec<_>>(), [Target::Prev, Target::Next]);
        for &(t, r) in &cs {
            assert_eq!(target_at(&s, mid(r).0, mid(r).1), Some(t), "{w}x{h}");
        }
        assert!(cs[0].1.x + cs[0].1.w <= cs[1].1.x, "prev ends before next in {w}x{h}");
        assert_eq!(controls(&single(w, h, 1)).count(), 0, "one image steps nowhere");
        assert_eq!(controls(&Shown { gallery: true, ..s }).count(), 0, "the gallery has none");
    }
}

/// The hit test nav.rs had before the shared `Rect`, written out again.
fn old_hit(x: i32, y: i32, vw: u32, vh: u32) -> Option<i32> {
    let top = (vh.saturating_sub(64) / 2) as i32;
    let inside = |rx: i32| x >= rx && x < rx + 40 && y >= top && y < top + 64;
    let sides = [(8, -1), (vw.saturating_sub(48) as i32, 1)];
    sides.into_iter().find(|&(rx, _)| inside(rx)).map(|(_, d)| d)
}

#[test]
fn a_click_steps_where_it_stepped_before() {
    for (w, h) in SIZES {
        let s = single(w, h, 2);
        for y in (-2..h as i32 + 2).step_by(3) {
            for x in -2..w as i32 + 2 {
                let now = target_at(&s, x, y).and_then(Target::step);
                assert_eq!(now, old_hit(x, y, w, h), "({x}, {y}) in {w}x{h}");
            }
        }
    }
}
