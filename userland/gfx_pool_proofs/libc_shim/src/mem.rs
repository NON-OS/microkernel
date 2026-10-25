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

//! Anonymous mappings as host memory. A mapping is handed out filled with
//! POISON, the pages a previous holder left as far as the driver can tell;
//! one unmapped is checked against what was mapped, whole.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

/// What a fresh mapping holds until the driver writes it.
pub const POISON: u8 = 0xA5;
const ENOMEM: isize = -12;

thread_local! {
    pub(crate) static MAPS: RefCell<BTreeMap<usize, Box<[u8]>>> = const { RefCell::new(BTreeMap::new()) };
    pub(crate) static REFUSE: Cell<bool> = const { Cell::new(false) };
    pub(crate) static WRONG: Cell<u32> = const { Cell::new(0) };
}

pub fn mk_mmap(_: *mut u8, len: usize, _: i32, _: i32, _: i32, _: i64) -> *mut u8 {
    if REFUSE.with(Cell::get) || len == 0 {
        return ENOMEM as *mut u8;
    }
    let mut pages = vec![POISON; len].into_boxed_slice();
    let base = pages.as_mut_ptr();
    MAPS.with(|m| m.borrow_mut().insert(base as usize, pages));
    base
}

/// An unmap of anything but a whole mapping is counted (`unmapped_wrong`).
pub fn mk_munmap(addr: *mut u8, len: usize) -> i64 {
    let whole = MAPS.with(|m| {
        let mut m = m.borrow_mut();
        let ok = m.get(&(addr as usize)).is_some_and(|p| p.len() == len);
        ok && m.remove(&(addr as usize)).is_some()
    });
    if whole {
        return 0;
    }
    WRONG.with(|w| w.set(w.get() + 1));
    -22
}
