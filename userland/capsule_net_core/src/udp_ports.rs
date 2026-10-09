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

use table::Ports;

pub const MAX_UDP_SOCKETS: usize = 16;

static TABLE: Mutex<Ports<SocketHandle, MAX_UDP_SOCKETS>> = Mutex::new(Ports::new());

pub fn insert(owner_pid: u32, local_port: u16, handle: SocketHandle) -> bool {
    TABLE.lock().insert(owner_pid, local_port, handle)
}

pub fn get(owner_pid: u32, local_port: u16) -> Option<SocketHandle> {
    TABLE.lock().get(owner_pid, local_port)
}

pub fn remove(owner_pid: u32, local_port: u16) {
    TABLE.lock().remove(owner_pid, local_port)
}

/// Take out the ports of every client `alive` says has ended.
pub fn take_ended(alive: impl Fn(u32) -> bool) -> Vec<SocketHandle> {
    TABLE.lock().take_ended(alive)
}

/// Forget every bound port. A rebuilt stack has a new socket set: a handle
/// from the old one names some other client's socket in it, or a socket of
/// the wrong kind, or none, and smoltcp panics on the last two.
pub fn forget_all() {
    TABLE.lock().forget_all()
}
