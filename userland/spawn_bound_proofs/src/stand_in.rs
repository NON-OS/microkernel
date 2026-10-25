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

//! What stands in for the kernel around the mounted bound: a process table
//! of capability masks, the calling pid, the serial line, a manifest's two
//! capability fields, and the attested parent the installer names.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

thread_local! {
    static CALLER: Cell<Option<u32>> = const { Cell::new(None) };
    static CAPS: RefCell<BTreeMap<u32, u64>> = const { RefCell::new(BTreeMap::new()) };
    static SERIAL: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Sets the calling pid and the processes that exist, with their masks.
pub fn world(caller: Option<u32>, procs: &[(u32, u64)]) {
    CALLER.set(caller);
    CAPS.with(|c| *c.borrow_mut() = procs.iter().copied().collect());
    SERIAL.with(|s| s.borrow_mut().clear());
}

pub fn said() -> String {
    SERIAL.with(|s| s.borrow().clone())
}

mod os;
pub use os::{process, security, sys, syscall};

#[derive(Clone, Copy)]
pub struct AttestedParent(pub u32);
impl AttestedParent {
    pub fn pid(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnError {
    Verify,
}
