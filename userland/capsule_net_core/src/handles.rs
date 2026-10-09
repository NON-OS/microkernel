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

mod table;

use alloc::vec::Vec;

use smoltcp::iface::SocketHandle;
use spin::Mutex;

use table::Handles;

pub const MAX_SOCKETS: usize = 32;

static TABLE: Mutex<Handles<SocketHandle, MAX_SOCKETS>> = Mutex::new(Handles::new());

pub fn alloc(owner_pid: u32, handle: SocketHandle) -> Option<u32> {
    TABLE.lock().alloc(owner_pid, handle)
}

pub fn get(index: u32, sender_pid: u32) -> Option<SocketHandle> {
    TABLE.lock().get(index, sender_pid)
}

pub fn free(index: u32, sender_pid: u32) {
    TABLE.lock().free(index, sender_pid)
}

/// Take out the connections of every client `alive` says has ended.
pub fn take_ended(alive: impl Fn(u32) -> bool) -> Vec<SocketHandle> {
    TABLE.lock().take_ended(alive)
}

/// Forget every connection. A rebuilt stack has a new socket set: a handle
/// from the old one names some other client's socket in it, or a socket of
/// the wrong kind, or none, and smoltcp panics on the last two.
pub fn forget_all() {
    TABLE.lock().forget_all()
}
