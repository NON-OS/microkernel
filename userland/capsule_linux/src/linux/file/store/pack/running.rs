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

//! The closure of the program this process runs (closure.rs), which the
//! pack table loads by. Set once the run knows its package; until then, and
//! for a program with no closure, every pack of the family is loaded.

use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use super::closure::serves;
use super::paths::package_of;
use crate::linux::file::family::Family;

struct Running(RefCell<Option<Vec<String>>>);
// SAFETY: the serve loop is the only thread that reads a guest's files.
unsafe impl Sync for Running {}

static RUNNING: Running = Running(RefCell::new(None));

/// Load only `closure`'s packs from here on, or every pack with None. The
/// packs loaded so far are dropped, so the next read loads by it.
pub fn run_closure(closure: Option<Vec<String>>) {
    *RUNNING.0.borrow_mut() = closure;
    super::table::forget();
}

/// Whether the store name `path` is a pack of `family` this run loads.
pub(super) fn loads(family: Family, path: &[u8]) -> bool {
    let closure = RUNNING.0.borrow();
    package_of(family, path).is_some_and(|p| serves(closure.as_deref(), p))
}
