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

//! The listeners net.sockets opened on this stack. Lock order: the stack
//! (`state::with_iface`), then this table, then the connection handles.

mod arm;
pub mod gate;
mod table;

use alloc::vec::Vec;

use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::tcp::Socket;
use spin::Mutex;

use crate::server::handlers::tcp::connect::new_socket;
use table::Listeners;
pub use table::Refusal;

/// Ports listened on at once. nsh.front holds one.
pub const MAX_LISTENERS: usize = 4;
/// Connections taken and not yet accepted, per listener.
pub const BACKLOG: usize = 4;

type Table = Listeners<SocketHandle, MAX_LISTENERS, BACKLOG>;

static TABLE: Mutex<Table> = Mutex::new(Listeners::new());

pub fn is_listener(handle: u32) -> bool {
    Table::is_listener(handle)
}

pub fn open(owner: u32, port: u16, socket: SocketHandle) -> Result<u32, Refusal> {
    TABLE.lock().open(owner, port, socket)
}

pub fn close(handle: u32, owner: u32) -> Option<Vec<SocketHandle>> {
    TABLE.lock().close(handle, owner)
}

/// The oldest connection `handle`'s queue holds whose handshake is done,
/// handed to `give`; when `give` refuses it, it goes back to the front.
/// `None` when `owner` holds no such listener, `Some(None)` when nothing
/// is ready.
pub fn accept<R>(
    sockets: &SocketSet<'static>,
    handle: u32,
    owner: u32,
    give: impl FnOnce(SocketHandle) -> Result<R, ()>,
) -> Option<Option<Result<R, ()>>> {
    let mut table = TABLE.lock();
    let l = table.get_mut(handle, owner)?;
    let Some(h) = l.take(|h| arm::ready(sockets.get::<Socket>(h).state())) else {
        return Some(None);
    };
    let given = give(h);
    if given.is_err() {
        let _ = l.restore(h);
    }
    Some(Some(given))
}

/// Whether `owner`'s listener has a connection ready to accept; `None`
/// when it holds no such listener.
pub fn readable(sockets: &SocketSet<'static>, handle: u32, owner: u32) -> Option<bool> {
    let mut table = TABLE.lock();
    let l = table.get_mut(handle, owner)?;
    Some(l.any_ready(|h| arm::ready(sockets.get::<Socket>(h).state())))
}

/// Run after every poll of the stack.
pub fn arm(sockets: &mut SocketSet<'static>) {
    TABLE.lock().for_each_mut(|l| {
        for h in arm::arm(l, sockets, new_socket) {
            sockets.remove(h);
        }
    });
}

pub fn any() -> bool {
    TABLE.lock().any()
}

pub fn take_ended(alive: impl Fn(u32) -> bool) -> Vec<SocketHandle> {
    TABLE.lock().take_ended(alive)
}

pub fn take_all() -> Vec<SocketHandle> {
    TABLE.lock().take_all()
}

pub fn forget_all() {
    TABLE.lock().forget_all()
}
