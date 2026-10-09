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

use core::sync::atomic::{AtomicU32, Ordering};

use super::io::ask_once;
use super::route::{insert, remove};

/// The bit every handle of a conversation with a proxy carries, and no
/// handle net.sockets gives out: it counts its handles up from 1, and one
/// that reached this bit is refused (`socket_open`) rather than mistaken.
pub const PROXIED: u32 = 0x8000_0000;

/// The number in the next conversation's handle.
///
/// Every conversation gets a handle of its own, so a close or a read for a
/// fetch that has already gone cannot reach the one that replaced it.
static NEXT: AtomicU32 = AtomicU32::new(1);

/// Whether `handle` names a conversation with a proxy.
pub fn is_proxied(handle: u32) -> bool {
    handle & PROXIED != 0
}

/// Begin a conversation with the proxy at `port`, on a stream of its own
/// (`streams`), so several run side by side. Its reset is asked here, once
/// and briefly; a proxy that does not answer in the poll wait is asked
/// again by the ticks that follow, before anything else goes. A call
/// refused outright (no proxy, a dead port) fails the open at once, as
/// does a proxy whose every stream the browser already holds.
pub fn open(port: u32) -> Result<u32, ()> {
    let n = (NEXT.fetch_add(1, Ordering::Relaxed) & !PROXIED).max(1);
    let handle = PROXIED | n;
    insert(handle, port).ok_or(())?;
    if ask_once(handle).is_err() {
        remove(handle, false);
        return Err(());
    }
    Ok(handle)
}

/// Let the conversation go. The proxy is told on a later tick, without
/// holding this one (`io::tell_reset`).
pub fn close(handle: u32) -> bool {
    remove(handle, true);
    true
}
