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

use super::call::exchange;
use super::frames::STREAM_RESET;
use super::route::with;

/// The handle the browser carries while proxied.
///
/// One conversation at a time per capsule: `net.socks5` keys its handshakes on
/// the sender the kernel attests, so the pid is the identity and a second
/// number here would add nothing.
const HANDLE: u32 = 1;

pub fn open() -> Result<u32, ()> {
    let port = with(|route| route.socks_port)?;
    with(|route| {
        route.pending.clear();
        route.closed = false;
        route.seq = 1;
    })?;
    /*
     * A refusal here is not fatal: the proxy may simply not have a
     * conversation to forget yet, which is the normal case for the first
     * page of a session.
     */
    let _ = exchange(port, &[STREAM_RESET], true);
    Ok(HANDLE)
}

/// Nothing is dialled here. The destination travels in the SOCKS CONNECT
/// request the browser sends next, and the proxy opens the tunnel when it
/// arrives.
pub fn connect() -> Result<(), ()> {
    Ok(())
}

pub fn close() -> bool {
    with(|route| {
        route.pending.clear();
        route.closed = false;
    })
    .is_ok()
}
