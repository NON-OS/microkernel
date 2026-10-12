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
//! Locks set from outside: written once the host is ready, then only
//! when they change, with the padding bits cleared.

use crate::sim::{keyboard_mouse, Run};
use nonos_bt_hogp::host::Step;
use nonos_bt_hogp::report::{CAPS_LOCK, SCROLL_LOCK};

#[test]
fn locks_set_before_ready_are_written_when_ready() {
    let db = keyboard_mouse();
    let out = db.handle_of(0x2a32);
    let mut run = Run::new(db);
    assert_eq!(run.host.set_locks(CAPS_LOCK), Step::default());
    run.go();
    // The battery's CCCD write comes after; the commands are the LEDs'.
    let last = run.peer.writes.iter().rfind(|w| w.2).cloned();
    assert_eq!(last, Some((out, vec![CAPS_LOCK], true)));
}

#[test]
fn locks_set_when_ready_are_written_on_change_only() {
    let db = keyboard_mouse();
    let out = db.handle_of(0x2a32);
    let mut run = Run::new(db);
    run.go();
    let write = |bits: u8| vec![[&[0x52][..], &out.to_le_bytes(), &[bits]].concat()];
    assert_eq!(run.host.set_locks(SCROLL_LOCK).send, write(SCROLL_LOCK));
    assert_eq!(run.host.set_locks(SCROLL_LOCK), Step::default());
    // Bits 5 to 7 are padding (HID 1.11, B.1) and never reach the device.
    assert_eq!(run.host.set_locks(0xe0 | CAPS_LOCK).send, write(CAPS_LOCK));
    assert_eq!(run.host.locks(), CAPS_LOCK);
}
