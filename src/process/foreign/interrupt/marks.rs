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

//! Which guest threads their supervisors have marked for a tick stop.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

use spin::Mutex;

static MARKED: Mutex<Vec<u32>> = Mutex::new(Vec::new());
/// Set while any thread is marked, so a tick with nothing marked costs one load.
static ANY: AtomicBool = AtomicBool::new(false);

pub(super) fn mark(pid: u32) {
    let mut marked = MARKED.lock();
    /*
     * Asked under the lock: a teardown drops the thread's registry row before
     * it forgets its mark, so a thread gone meanwhile is never left one.
     */
    if !crate::process::foreign::is_foreign(pid) {
        return;
    }
    if !marked.contains(&pid) {
        marked.push(pid);
    }
    ANY.store(true, Ordering::Release);
}

/// A thread that is gone keeps no mark for a later one with its pid.
pub(in crate::process::foreign) fn forget(pid: u32) {
    let mut marked = MARKED.lock();
    marked.retain(|&p| p != pid);
    ANY.store(!marked.is_empty(), Ordering::Release);
}

/// Whether any thread is marked, for the tick's one-load test.
pub(super) fn any() -> bool {
    ANY.load(Ordering::Acquire)
}

pub(super) fn take(pid: u32) -> bool {
    let mut marked = MARKED.lock();
    let Some(at) = marked.iter().position(|&p| p == pid) else {
        return false;
    };
    marked.swap_remove(at);
    ANY.store(!marked.is_empty(), Ordering::Release);
    true
}
