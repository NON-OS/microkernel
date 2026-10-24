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

//! The registry as a test sets it: which pid holds which name on which port,
//! and whether the caller holds the right to register at runtime.

use std::cell::{Cell, RefCell};

pub struct ServiceEndpoint {
    pub pid: u32,
    pub port: u32,
}

thread_local! {
    static HELD: RefCell<Vec<(&'static str, u32, u32)>> = const { RefCell::new(Vec::new()) };
    static RIGHT: Cell<bool> = const { Cell::new(false) };
}

/// `name` held by `pid` on `port` from now on.
pub fn hold(name: &'static str, pid: u32, port: u32) {
    HELD.with(|h| h.borrow_mut().push((name, pid, port)));
}

pub fn set_right(on: bool) {
    RIGHT.with(|r| r.set(on));
}

pub fn lookup_service(name: &str) -> Option<ServiceEndpoint> {
    HELD.with(|h| {
        h.borrow().iter().find(|e| e.0 == name).map(|e| ServiceEndpoint { pid: e.1, port: e.2 })
    })
}

pub fn caller_has_register_right() -> bool {
    RIGHT.with(Cell::get)
}
