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

//! Windows a departed device's holder may still have mapped. A device that
//! leaves while held cannot have its pages unmapped from here: they live in
//! the holder's address space, and only that space's own context may walk
//! it. So its grants are dropped and its windows fenced: no later device
//! whose registers fall in one is admitted until the space that maps it
//! is gone, or the holder would reach the newcomer's registers.

use alloc::vec::Vec;
use spin::Mutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fenced {
    pub pid: u32,
    pub start: u64,
    pub len: u64,
}

static FENCED: Mutex<Vec<Fenced>> = Mutex::new(Vec::new());

/// Whether [a, a + alen) and [b, b + blen) share a byte. Pure.
pub fn overlaps(a: u64, alen: u64, b: u64, blen: u64) -> bool {
    alen != 0 && blen != 0 && a < b.saturating_add(blen) && b < a.saturating_add(alen)
}

pub(in crate::hardware::broker) fn fence(f: Fenced) {
    let mut v = FENCED.lock();
    if !v.contains(&f) {
        v.push(f);
    }
}

/// The holder whose fence a window falls in, if any.
#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
pub(in crate::hardware::broker) fn fenced_by(start: u64, len: u64) -> Option<u32> {
    FENCED.lock().iter().find(|f| overlaps(f.start, f.len, start, len)).map(|f| f.pid)
}

/// A process's tables are freed (`heir` None) or handed to another thread
/// of its group (`heir` its pid), which keeps the mappings and so the fence.
pub fn after_exit(pid: u32, heir: Option<u32>) {
    let mut v = FENCED.lock();
    match heir {
        None => v.retain(|f| f.pid != pid),
        Some(h) => v.iter_mut().filter(|f| f.pid == pid).for_each(|f| f.pid = h),
    }
}
