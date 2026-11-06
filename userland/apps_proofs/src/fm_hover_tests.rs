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

//! The file manager's hover, fed as `app.rs` feeds it: a move inside one target
//! repaints nothing, one onto another repaints, and a scrolled listing names
//! the entries it draws.

use nonos_app_skeleton::InputEvent;
use nonos_app_skeleton::InputKind::{self, PointerAbs};

use crate::fm::browse_hit::browse_at;
use crate::fm::hover::Hover;
use crate::fm::layout::{content_x, FIRST_ROW_Y, GRID_TOP};
use crate::fm::list_cols::HEAD_H;
use crate::fm::pane::Pane;
use crate::fm::target::Target;
use crate::fm_check::points;
use crate::fm_controls_tests::{drawn, pane};

pub fn at(p: &Pane, x: i32, y: i32) -> Option<Target> {
    (x >= 0 && y >= 0).then(|| browse_at(p, x as u32, y as u32)).flatten()
}

/// Feeds the pointer, then names the target under it again; true to repaint.
pub fn feed(h: &mut Hover, p: &Pane, kind: InputKind, (x, y): (u32, u32)) -> bool {
    let (x, y) = if kind == PointerAbs && x == u32::MAX { (-1, -1) } else { (x as i32, y as i32) };
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    let held = h.feed(&InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns });
    h.settle(h.pointer.position().and_then(|(x, y)| at(p, x, y))) || held
}

pub const LEAVE: (u32, u32) = (u32::MAX, u32::MAX);

#[test]
fn a_move_inside_a_target_repaints_nothing_and_one_onto_the_next_repaints() {
    for (grid, scroll) in [(false, 0), (false, 9), (true, 0), (true, 11)] {
        let (p, mut hv) = (pane(1100, 700, grid, scroll), Hover::new());
        for &(t, r) in drawn(&p).iter().rev().take(6) {
            let [mid, a, b, c, d] = points(r);
            assert!(feed(&mut hv, &p, PointerAbs, a), "onto {t:?}");
            assert!([mid, b, c, d].iter().all(|&q| !feed(&mut hv, &p, PointerAbs, q)), "in {t:?}");
            assert!(hv.over == Some(t) && hv.look(t, |x, y| at(&p, x, y)).hovered);
        }
    }
}

#[test]
fn a_scrolled_listing_names_the_entries_it_draws() {
    let (list, mut hv) = (pane(1100, 700, false, 9), Hover::new());
    assert!(feed(&mut hv, &list, PointerAbs, (content_x() + 200, FIRST_ROW_Y + HEAD_H + 1)));
    assert_eq!(hv.over, Some(Target::Row(9)));
    let grid = pane(1100, 700, true, 11);
    assert!(feed(&mut hv, &grid, PointerAbs, (content_x() + 1, GRID_TOP + 1)));
    assert_eq!(hv.over, Some(Target::Cell(11 - 11 % grid.grid_cols as usize)));
}
