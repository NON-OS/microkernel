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

//! What real descriptors are walked into: media keys under report IDs, a
//! report ID mouse, a touchscreen, and a mouse whose reports outgrow a read.

use super::layout;
use super::vectors::MEDIA_KEYS;
use super::vectors_more::{ID_MOUSE, LONG_MOUSE, TOUCHSCREEN};
use crate::descriptors::report::NONE;

#[test]
fn media_keys_come_under_their_own_report_ids() {
    let l = layout(MEDIA_KEYS);
    let [consumer, system] = l.media.map(|m| m.expect("both parts"));
    assert_eq!((consumer.id, system.id, l.ids, l.longest), (2, 3, true, 3));
    let s = consumer.slots.expect("one usage slot");
    assert_eq!((s.at, s.size, s.count), (0, 16, 1));
    assert_eq!(consumer.values, [Some(0xe2), Some(0xe9), Some(0xea), None]);
    assert_eq!(system.bits, [NONE, NONE, NONE, 0], "Power Down is the first system bit");
}

#[test]
fn a_report_id_mouse_and_a_touchscreen_are_pointers_of_their_report() {
    let m = layout(ID_MOUSE).pointer.expect("mouse");
    let (x, y) = (m.x.unwrap(), m.y.unwrap());
    assert_eq!(
        (m.id, m.buttons, x.at, y.at, x.size, x.relative),
        (2, [0, 1, 2, 3, 4], 8, 20, 12, true)
    );
    assert_eq!((x.min, x.max), (-2047, 2047));
    let t = layout(TOUCHSCREEN);
    let p = t.pointer.expect("touchscreen");
    assert_eq!((p.id, p.tip, p.range, p.buttons[0], t.longest), (1, 0, 1, NONE, 6));
    assert_eq!(p.x.map(|x| (x.at, x.max, x.relative)), Some((8, 0xfff, false)));
}

#[test]
fn a_mouse_whose_reports_outgrow_one_read_says_so() {
    let l = layout(LONG_MOUSE);
    assert!(l.pointer.is_some());
    assert_eq!(l.longest, 9);
}
