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

//! Each server's request ids: the name MkIpcRecvReq gives a request and
//! MkIpcReplyTo answers by. Per server, from 1, never reused while the
//! server lives, so a stale id answers nothing. Not the call's token,
//! which counts every call on the machine and would tell each server how
//! busy every other capsule is. Taken only under PENDING (push.rs,
//! clear_pid.rs).

use alloc::collections::BTreeMap;
use spin::Mutex;

static NEXT: Mutex<BTreeMap<u32, u64>> = Mutex::new(BTreeMap::new());

/// The id of the next request queued on `server`.
pub(super) fn next(server: u32) -> u64 {
    let mut map = NEXT.lock();
    let last = map.entry(server).or_insert(0);
    *last = last.wrapping_add(1).max(1);
    *last
}

/// `pid` is dead: its ids go, so a process given the pid starts at 1.
pub(super) fn forget(pid: u32) {
    NEXT.lock().remove(&pid);
}
