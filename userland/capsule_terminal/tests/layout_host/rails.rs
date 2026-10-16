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

//! Splitting the width between the left rail and the body.

use crate::layout::limits::LEFT_RAIL_W;
use crate::support::{lay, BOTH_RAILS, NO_RAILS};

#[test]
fn the_body_spans_the_full_width_without_rails() {
    let l = lay(1440, 900, NO_RAILS);
    assert_eq!(l.body.x, 0);
    assert_eq!(l.body.w, 1440);
    assert_eq!(l.left_rail.w, 0);
}

/// The two content columns must account for the whole width, or the
/// rightmost pixels of the window are never painted by anyone.
#[test]
fn the_rails_and_the_body_sum_to_the_width() {
    let l = lay(1440, 900, BOTH_RAILS);
    assert_eq!(l.left_rail.w + l.body.w, 1440);
    assert_eq!(l.body.x, l.left_rail.w);
    assert_eq!(l.body.x + l.body.w, 1440);
}

/// The rail hosts a four-column process table drawn at ttf::MIN_UI_PX, so its
/// width is a hard requirement, not a taste: PID 40 + CPU 46 + MEM 46 + one
/// 8px gutter + 12px padding either side leaves 156px for the capsule name.
#[test]
fn the_left_rail_fits_the_process_table() {
    const PID_W: u32 = 40;
    const CPU_W: u32 = 46;
    const MEM_W: u32 = 46;
    const RAIL_GAP: u32 = 8;
    const RAIL_PAD: u32 = 12;
    let content = LEFT_RAIL_W - RAIL_PAD * 2;
    let name_w = content - (PID_W + CPU_W + MEM_W + RAIL_GAP);
    assert!(name_w >= 150, "name column starved at {}", name_w);
    let l = lay(1280, 720, BOTH_RAILS);
    assert_eq!(l.left_rail.w, LEFT_RAIL_W, "the rail is not clipped at 1280");
    assert_eq!(l.left_rail.x, 0);
}

/// The rail and the body may never share a pixel column: the body starts where
/// the rail ends, at every width the rail survives.
#[test]
fn the_body_never_overlaps_the_left_rail() {
    for w in [640u32, 700, 900, 1024, 1280, 1440, 2560] {
        let l = lay(w, 720, BOTH_RAILS);
        assert_eq!(l.body.x, l.left_rail.x + l.left_rail.w, "overlap at {}", w);
        assert_eq!(l.input.x, l.body.x, "input drifted from the body at {}", w);
        assert_eq!(l.left_rail.w + l.body.w, w, "width unaccounted at {}", w);
    }
}
