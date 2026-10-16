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

//! Section heights, content height and the clamped offset.

use crate::paint::rail_geom::{disk_h, net_h, sys_h, telemetry_h};
use crate::paint::rail_left_geom::nav_h;
use crate::paint::rail_scroll::{clamp, content_h, telemetry_top, RailFit};
use crate::paint::rail_text::{lh, RAIL_GAP, RAIL_PAD};
use crate::support::{fit, RAIL};

#[test]
fn the_section_heights_compose_into_the_telemetry_block() {
    assert_eq!(sys_h(), 34 + lh() * 5 + 3 + RAIL_GAP * 2 + 30);
    assert_eq!(net_h(), 34 + lh() * 4);
    assert_eq!(disk_h(), 34 + lh() * 2);
    let want = sys_h() + net_h() + disk_h() + RAIL_GAP * 2 + RAIL_PAD;
    assert_eq!(telemetry_h(), want, "the composer steps by exactly these heights");
}

#[test]
fn content_height_is_the_navigation_lists_plus_the_telemetry_block() {
    let f = fit(4, 3);
    assert_eq!(content_h(f), nav_h(4, 3) + RAIL_GAP + telemetry_h());
    let off = RailFit { telemetry: false, ..f };
    assert_eq!(content_h(off), nav_h(4, 3), "no telemetry, no telemetry height");
    assert_eq!(telemetry_top(0, f), (nav_h(4, 3) + RAIL_GAP) as i32);
    assert_eq!(telemetry_top(90, f), (nav_h(4, 3) + RAIL_GAP) as i32 - 90);
}

#[test]
fn the_offset_is_clamped_at_both_ends() {
    let f = fit(30, 20);
    let content = content_h(f);
    assert!(content > RAIL.h, "the fixture must actually overflow the rail");
    assert_eq!(clamp(0, f, RAIL.h), 0);
    assert_eq!(clamp(u32::MAX, f, RAIL.h), content - RAIL.h);
    assert_eq!(clamp(content - RAIL.h + 1, f, RAIL.h), content - RAIL.h);
    let short = fit(1, 0);
    assert_eq!(clamp(500, short, 4000), 0, "content shorter than the rail never scrolls");
}
