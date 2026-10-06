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

//! Config space reads. The I219 ring-flush check reads one word,
//! PCICFG_DESC_RING_STATUS at 0xE4; the test sets what it holds, or a
//! refusal, and can see which reads were made.

use std::cell::{Cell, RefCell};

thread_local! {
    static RING_STATUS: Cell<i64> = const { Cell::new(0) };
    static READS: RefCell<Vec<(u32, u32)>> = const { RefCell::new(Vec::new()) };
}

/// What the next reads of offset 0xE4 answer: a word, or a negative errno.
pub fn set_ring_status(v: i64) {
    RING_STATUS.with(|s| s.set(v));
}

/// Every (offset, width) read on this thread.
pub fn config_reads() -> Vec<(u32, u32)> {
    READS.with(|r| r.borrow().clone())
}

pub fn mk_pci_config_read(_device_id: u64, _epoch: u64, offset: u32, width: u32) -> i64 {
    READS.with(|r| r.borrow_mut().push((offset, width)));
    if offset == 0xE4 && width == 2 {
        RING_STATUS.with(|s| s.get())
    } else {
        -22
    }
}
