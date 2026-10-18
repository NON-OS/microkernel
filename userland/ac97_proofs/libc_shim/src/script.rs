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

//! How the broker answers this thread, and every call it was asked.

use std::cell::RefCell;

/// How the broker answers this thread. Port grant ids are 50 plus the BAR.
#[derive(Clone, Copy)]
pub struct Script {
    pub claim: i64,
    pub command: i64,
    /// Each BAR's window size; zero refuses the grant.
    pub ports: [u16; 2],
}

impl Default for Script {
    fn default() -> Self {
        Self { claim: 7, command: 0, ports: [256, 64] }
    }
}

thread_local! {
    static SCRIPT: RefCell<Script> = RefCell::new(Script::default());
    static CALLS: RefCell<Vec<(&'static str, u64)>> = const { RefCell::new(Vec::new()) };
}

pub fn script(s: Script) {
    SCRIPT.with(|c| *c.borrow_mut() = s);
}

pub(crate) fn note(call: &'static str, id: u64) {
    CALLS.with(|c| c.borrow_mut().push((call, id)));
}

/// Every broker call made on this thread, as (call, id), in order.
pub fn calls() -> Vec<(&'static str, u64)> {
    CALLS.with(|c| c.borrow().clone())
}

pub(crate) fn answer() -> Script {
    SCRIPT.with(|s| *s.borrow())
}
