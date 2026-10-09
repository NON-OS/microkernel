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
//! Who is waiting on a kernel-owned reply inbox.
//!
//! A kernel caller driving a round trip to a capsule waits on a reply inbox
//! the kernel owns, and a kernel-owned inbox has no process to wake. The
//! caller used to sleep a scheduler tick per round and look again, so every
//! request to a driver capsule cost at least a tick. The caller now names
//! itself here before it waits, and the reply that lands wakes it.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use spin::Mutex;

static WAITERS: Mutex<BTreeMap<String, u32>> = Mutex::new(BTreeMap::new());

/// `pid` waits on `module` until `unwait`.
pub fn wait_on(module: &str, pid: u32) {
    WAITERS.lock().insert(String::from(module), pid);
}

pub fn unwait(module: &str) {
    WAITERS.lock().remove(module);
}

/// Wake whoever waits on `module`, after a message was put there.
pub fn wake_waiter(module: &str) {
    let pid = WAITERS.lock().get(module).copied();
    if let Some(pid) = pid {
        crate::sched::wake_process(pid);
    }
}
