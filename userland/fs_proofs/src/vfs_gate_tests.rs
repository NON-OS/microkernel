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

//! Who vfs answers (`capsule_vfs/src/server/fs_gate`). vfs served any
//! process that found its port, so FileSystem, the bit the kernel grants for
//! the store, decided nothing. It now serves the kernel-internal client
//! (sender pid 0) and a sender the kernel says holds FileSystem, asking on
//! every request. The decision is the shipping rule, included by path, with
//! the kernel's answer played by a closure.

use std::cell::{Cell, RefCell};

#[path = "../../capsule_vfs/src/server/fs_gate/rule.rs"]
mod rule;

use rule::allows;

/// The request dispatch, as vfs ships it.
const DISPATCH: &str = include_str!("../../capsule_vfs/src/server/dispatch.rs");

#[test]
fn the_kernel_client_is_served_without_asking() {
    let asked = Cell::new(0);
    assert!(allows(0, |_| {
        asked.set(asked.get() + 1);
        false
    }));
    assert_eq!(asked.get(), 0, "pid 0 is the kernel's own client and is not asked about");
}

#[test]
fn a_holder_is_served_and_any_other_sender_is_refused() {
    for pid in [1u32, 2, 7, 4104, 65_535, u32::MAX] {
        assert!(allows(pid, |_| true), "holder {pid}");
        assert!(!allows(pid, |_| false), "non-holder {pid}");
    }
}

#[test]
fn the_kernel_is_asked_about_the_sender_itself() {
    for pid in [1u32, 42, 900, u32::MAX] {
        let seen = Cell::new(0u32);
        allows(pid, |asked| {
            seen.set(asked);
            true
        });
        assert_eq!(seen.get(), pid);
    }
}

#[test]
fn each_request_asks_once_and_no_verdict_outlives_it() {
    // pid 9 holds FileSystem, exits, and the pid is handed to a process
    // without it: the verdict follows the kernel's answer, not the past one.
    let answers = RefCell::new(vec![true, true, false, false, true].into_iter());
    let asked = Cell::new(0);
    let verdicts: Vec<bool> = (0..5)
        .map(|_| {
            allows(9, |_| {
                asked.set(asked.get() + 1);
                answers.borrow_mut().next().unwrap()
            })
        })
        .collect();
    assert_eq!(verdicts, [true, true, false, false, true]);
    assert_eq!(asked.get(), 5, "one question per request");
}

#[test]
fn dispatch_asks_the_gate_before_anything_else() {
    let start = DISPATCH.find("pub fn dispatch(").expect("vfs dispatch moved");
    let body = &DISPATCH[start..];
    let gate = body.find("fs_gate::refusal(&req, sender_pid)").expect("dispatch no longer asks the gate");
    let refused = body[gate..].find("return refused;").expect("a refusal is not returned");
    for later in ["generation::bump", "match req.op", "handlers::"] {
        let at = body.find(later).expect(later);
        assert!(gate + refused < at, "{later} runs before the gate refuses");
    }
}
