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

//! What the kernel says a process holds: set by the test, per thread.

use std::cell::RefCell;

thread_local! {
    static CAPS: RefCell<Vec<(u32, u64)>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_cap_check(pid: u32, mask: u64) -> bool {
    CAPS.with(|c| c.borrow().iter().any(|&(p, m)| p == pid && m & mask == mask))
}

/// From now on the kernel says `pid` holds `mask`.
pub fn grant_cap(pid: u32, mask: u64) {
    CAPS.with(|c| c.borrow_mut().push((pid, mask)));
}
