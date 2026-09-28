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

//! Writing to and reading from the proxy as the socket calls expect.

use super::frames::{absorb, ask};
use super::route::with;

/// Write bytes to the proxy and keep whatever it answers for the next read.
pub fn send(payload: &[u8]) -> Result<(), ()> {
    let port = with(|route| route.socks_port)?;
    let reply = ask(port, payload)?;
    absorb(&reply)
}

/// Take what the proxy has already answered.
///
/// Zero means nothing is waiting, which is what the callers above expect from
/// a socket that has not been spoken to yet.
///
/// An empty exchange is how the proxy is asked whether more has arrived. A
/// reply crosses several hops with a delay chosen at each one, so it is
/// almost never ready inside the same call that sent the request. Reading
/// only what a send happened to bring back meant the answer to every request
/// arrived after the only chance to collect it.
pub fn recv(out: &mut [u8]) -> Result<usize, ()> {
    let port = with(|route| route.socks_port)?;
    /*
     * Nothing held and the tunnel still open means the answer may simply not
     * have arrived yet, so ask. A closed tunnel has nothing more to give and
     * asking again would only stall the reader.
     */
    if with(|route| route.pending.is_empty() && !route.closed)? {
        if let Ok(more) = ask(port, &[]) {
            absorb(&more)?;
        }
    }
    with(|route| {
        let n = out.len().min(route.pending.len());
        out[..n].copy_from_slice(&route.pending[..n]);
        route.pending.drain(..n);
        n
    })
}
