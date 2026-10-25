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

//! The calls a dead server abandoned, and the servers that are gone. A
//! caller asleep on a reply the server will never send is woken and finds
//! its token here, so its call returns ESRCH at once; a call that looked the
//! server up just before its names went finds the server here and is
//! refused before it waits. A server stays here until its tables are freed,
//! by which time no lookup can return it.

use alloc::collections::BTreeSet;
use spin::Mutex;

static ABANDONED: Mutex<BTreeSet<(u32, u64)>> = Mutex::new(BTreeSet::new());
static GONE: Mutex<BTreeSet<u32>> = Mutex::new(BTreeSet::new());

/// `server` is dead and these (caller pid, token) calls will get no reply.
pub(super) fn record(server: u32, owed: &[(u32, u64)]) {
    GONE.lock().insert(server);
    ABANDONED.lock().extend(owed.iter().copied());
}

/// Whether the call `token` of `caller` was abandoned; forgets it if so.
pub(crate) fn take(caller: u32, token: u64) -> bool {
    ABANDONED.lock().remove(&(caller, token))
}

/// Whether `server` died and is not yet freed.
pub(super) fn gone(server: u32) -> bool {
    GONE.lock().contains(&server)
}

/// `pid`'s tables are freed: forget it as a server and as a caller.
pub(crate) fn forget(pid: u32) {
    GONE.lock().remove(&pid);
    ABANDONED.lock().retain(|(caller, _)| *caller != pid);
}
