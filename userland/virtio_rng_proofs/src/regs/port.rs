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

//! A legacy virtio device behind a port grant, as far as a reset and the
//! request queue reach it: the status byte, a reset it takes `held` reads
//! to finish (the old status still answering until then), every status
//! access kept in order, and the queue's size and page, which its reset
//! leaves as they were. One per test thread.

use std::cell::RefCell;

#[derive(Default)]
pub struct Port {
    pub status: u8,
    /// Reads after a reset that still answer the status from before it.
    pub held: u32,
    /// The status a reset found.
    pub(super) stale: u8,
    pub log: Vec<Status>,
    /// The queue selected, QueueNum for the request queue, and its page.
    pub sel: u16,
    pub qnum: u16,
    pub pfn: u32,
}

/// One access to the status register, and the value it carried.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Read(u8),
    Write(u8),
}

thread_local!(static PORT: RefCell<Port> = RefCell::new(Port::default()));

/// Run `f` on this thread's port device.
pub fn port<T>(f: impl FnOnce(&mut Port) -> T) -> T {
    PORT.with(|p| f(&mut p.borrow_mut()))
}
