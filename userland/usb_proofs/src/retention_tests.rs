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

//! Nothing the driver posts stays with it. A keyboard kept its
//! first 64 key events, ASCII byte and modifiers with each, in a queue only
//! a poll op drained, and no one called it: the boot password stayed in the
//! capsule for its lifetime. A keyboard or pointer now holds only what is
//! down, and no op feeds a report in or reads a key back out.

use crate::hid::{Keyboard, Mouse, Tablet};
use nonos_libc::{reset, take_posted, INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

#[test]
fn no_keyboard_or_pointer_owns_memory_a_key_log_could_sit_in() {
    // A queue of events needs a heap buffer, and so drop glue.
    assert!(!core::mem::needs_drop::<Keyboard>(), "a keyboard owns heap memory");
    assert!(!core::mem::needs_drop::<Mouse>(), "a mouse owns heap memory");
    assert!(!core::mem::needs_drop::<Tablet>(), "a tablet owns heap memory");
    // The keys down, their codes and the repeat; no room for an event log.
    let size = core::mem::size_of::<Keyboard>();
    assert!(size <= 256, "a keyboard is {size} bytes");
}

#[test]
fn every_key_typed_is_posted_once_down_and_once_up() {
    reset();
    let mut kbd = Keyboard::new();
    // h u n t e r 2, a hundred times.
    let typed = [0x0b, 0x18, 0x11, 0x17, 0x08, 0x15, 0x1f];
    for _ in 0..100 {
        for key in typed {
            kbd.feed(&[0, 0, key, 0, 0, 0, 0, 0]);
            kbd.feed(&[0; 8]);
        }
    }
    let posted = take_posted();
    let count = |kind| posted.iter().filter(|e| e.kind == kind).count();
    assert_eq!((count(INPUT_KIND_KEY_DOWN), count(INPUT_KIND_KEY_UP)), (700, 700));
}

#[test]
fn no_op_feeds_a_report_in_or_reads_a_key_back_out() {
    let dispatch = include_str!("../../capsule_driver_usb_hid/src/server/dispatch.rs");
    let ops = include_str!("../../capsule_driver_usb_hid/src/protocol/ops.rs");
    for gone in ["OP_FEED_KEYBOARD_REPORT", "OP_FEED_MOUSE_REPORT", "OP_POLL_KEYS", "OP_POLL_MOUSE"]
    {
        assert!(!dispatch.contains(gone) && !ops.contains(gone), "{gone} is still served");
    }
    use crate::usb_hid_wire::{OP_GET_STATE, OP_HEALTHCHECK, OP_PROBE_CONFIG};
    assert_eq!([OP_HEALTHCHECK, OP_PROBE_CONFIG, OP_GET_STATE], [1, 2, 7]);
}
