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

//! Stands in for `mk_caps_of`: the kernel's answer for a sender, from a table
//! of live pids and their masks, with every question asked kept for the test.

use std::cell::RefCell;

/// As libc names them (`libc/src/process.rs`); `caps_of_glue_tests` holds
/// both to the kernel's own bits.
pub const CAPS_OF_SPAWN_WINDOW: u64 = 1 << 24;
pub const CAPS_OF_APP_INSTALL: u64 = 1 << 30;

thread_local! {
    static HELD: RefCell<Vec<(u32, u64)>> = const { RefCell::new(Vec::new()) };
    static ASKED: RefCell<Vec<(u32, u64, u64)>> = const { RefCell::new(Vec::new()) };
}

/// The live pids and the masks the kernel would answer from.
pub fn hold(procs: &[(u32, u64)]) {
    HELD.with(|h| *h.borrow_mut() = procs.to_vec());
    ASKED.with(|a| a.borrow_mut().clear());
}

/// Every (pid, bit, sent_ms) asked since `hold`.
pub fn asked() -> Vec<(u32, u64, u64)> {
    ASKED.with(|a| a.borrow().clone())
}

pub fn mk_caps_of(pid: u32, bit: u64, sent_ms: u64) -> i64 {
    ASKED.with(|a| a.borrow_mut().push((pid, bit, sent_ms)));
    match HELD.with(|h| h.borrow().iter().find(|(p, _)| *p == pid).map(|(_, m)| *m)) {
        Some(mask) => (mask & bit != 0) as i64,
        None => -3,
    }
}
