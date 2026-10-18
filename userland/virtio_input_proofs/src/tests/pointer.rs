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

//! A finished report goes onto the ring as the router expects it: the
//! position first, then motion, the wheel, and each button in turn.

use crate::events::pointer::Pointer;
use crate::events::post_pointer;
use nonos_libc::posted;

#[test]
fn a_report_posts_position_motion_wheel_then_buttons() {
    let mut p = Pointer::empty();
    p.set_abs(100, None, true);
    p.add_rel(4, -2);
    p.add_wheel(-1);
    p.button(1, true);
    p.button(3, false);
    post_pointer(&p);
    let got: Vec<(u16, u32, i32, i32, i32, i32)> =
        posted().iter().map(|e| (e.kind, e.code, e.x, e.y, e.delta_x, e.delta_y)).collect();
    let want = vec![
        (3, 0, 100, 0, 0, 0),
        (2, 0, 0, 0, 4, -2),
        (4, 0, 0, 0, 0, -1),
        (5, 1, 0, 0, 0, 0),
        (6, 3, 0, 0, 0, 0),
    ];
    assert_eq!(got, want);
}

#[test]
fn an_empty_report_posts_nothing_and_extra_buttons_are_dropped() {
    post_pointer(&Pointer::empty());
    assert!(posted().is_empty());
    let mut p = Pointer::empty();
    for n in 0..20 {
        p.button(n, true);
    }
    post_pointer(&p);
    assert_eq!(posted().len(), 8);
}
