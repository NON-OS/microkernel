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

//! The poll: a host move read and posted in the absolute range, once per
//! position; nothing read while nothing is pending; nothing posted with
//! the host's integration off.

use nonos_libc::{posted, INPUT_ABS_MAX, INPUT_KIND_POINTER_ABS};

use super::util::up;
use crate::serve::step;

#[test]
fn a_host_move_is_posted_scaled_to_the_absolute_range() {
    let (mut d, host) = up();
    for (x, y, want) in [
        (0, 0, (0, 0)),
        (0xFFFF, 0xFFFF, (INPUT_ABS_MAX, INPUT_ABS_MAX)),
        (0x8000, 0x4000, (0x3FFF, 0x1FFF)),
    ] {
        host.borrow_mut().host_move(x, y);
        assert!(step(&mut d));
        let ev = posted();
        assert_eq!(ev.len(), 1);
        assert_eq!((ev[0].kind, ev[0].x, ev[0].y), (INPUT_KIND_POINTER_ABS, want.0, want.1));
        assert!(!host.borrow().pending_flag(), "the event was acknowledged");
    }
}

#[test]
fn nothing_pending_is_one_look_at_the_page_and_no_request() {
    let (mut d, host) = up();
    let before = host.borrow().requests.len();
    assert!(!step(&mut d));
    assert_eq!(host.borrow().requests.len(), before);
}

#[test]
fn the_same_position_is_posted_once() {
    let (mut d, host) = up();
    host.borrow_mut().host_move(100, 200);
    assert!(step(&mut d));
    host.borrow_mut().host_move(100, 200);
    assert!(step(&mut d));
    assert_eq!(posted().len(), 1);
}
