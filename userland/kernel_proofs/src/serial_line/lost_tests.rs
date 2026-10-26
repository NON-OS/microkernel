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

//! Once a fatal line has waited its whole bound on a lock that never freed,
//! the lines after it try once, not the whole bound again; and the first
//! line that does get the lock puts the full bound back.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use super::fatal::write_bounded;

/// How many times one fatal line asked for `port`, and what it wrote.
fn tries_for(port: &Mutex<()>, lost: &AtomicBool) -> (u32, Vec<u8>) {
    let (mut asked, mut out) = (0, Vec::new());
    let lock = || {
        asked += 1;
        port.try_lock().ok()
    };
    write_bounded(64, lost, lock, |b| out.push(b), &[b"[PANIC] x"]);
    (asked, out)
}

#[test]
fn a_lost_lock_is_waited_on_once_then_tried_once() {
    let (port, lost) = (Mutex::new(()), AtomicBool::new(false));
    let held = port.lock().unwrap();
    assert_eq!(tries_for(&port, &lost).0, 64, "the first line waits the bound out");
    assert!(lost.load(Ordering::Relaxed), "and says the lock is lost");
    let (asked, out) = tries_for(&port, &lost);
    assert_eq!(asked, 1, "the next line tries once");
    assert_eq!(out, b"[PANIC] x\r\n", "and is still written whole");
    drop(held);
    assert_eq!(tries_for(&port, &lost).0, 1, "a free lock is got at once");
    assert!(!lost.load(Ordering::Relaxed), "and the full bound is back");
}

#[test]
fn no_tries_at_all_never_marks_the_lock_lost() {
    // aarch64 with the MMU off asks for no lock; that is not a lost lock.
    let (port, lost) = (Mutex::new(()), AtomicBool::new(false));
    let mut out = Vec::new();
    write_bounded(0, &lost, || port.try_lock().ok(), |b| out.push(b), &[b"x"]);
    assert!(!lost.load(Ordering::Relaxed));
    assert_eq!(out, b"x\r\n");
}
