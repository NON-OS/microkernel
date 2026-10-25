// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The function's config space as a test sets it, and every read made of it.

use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static SPACE: RefCell<HashMap<u32, i64>> = RefCell::new(HashMap::new());
    static READS: RefCell<Vec<(u32, u32)>> = const { RefCell::new(Vec::new()) };
}

/// Answer a read at `offset` with `value` (a negative errno to refuse it).
pub fn set_config(offset: u32, value: i64) {
    SPACE.with(|s| s.borrow_mut().insert(offset, value));
}

/// Every (offset, width) this thread's driver read, in order.
pub fn config_reads() -> Vec<(u32, u32)> {
    READS.with(|r| r.borrow().clone())
}

pub(crate) fn read(offset: u32, width: u32) -> i64 {
    READS.with(|r| r.borrow_mut().push((offset, width)));
    SPACE.with(|s| s.borrow().get(&offset).copied().unwrap_or(0))
}
