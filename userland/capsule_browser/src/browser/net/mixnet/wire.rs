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

use alloc::vec::Vec;

use super::call::exchange;
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
    // A refusal here is not fatal: the proxy may simply not have a
    // conversation to forget yet, which is the normal case for the first
    // page of a session.
    let _ = exchange(port, &[STREAM_RESET], true);
    Ok(HANDLE)
}

/// Nothing is dialled here. The destination travels in the SOCKS CONNECT
/// request the browser sends next, and the proxy opens the tunnel when it
/// arrives.
pub fn connect() -> Result<(), ()> {
    Ok(())
}

/// The proxy marks every answer, so that an answer carrying no bytes is still
/// an answer. Without it "nothing yet" and "no reply at all" are the same
/// thing on the wire, and the caller waits out a timeout to tell them apart.
const STREAM_CLOSED: u8 = 1;

/// Tell the proxy to forget the previous conversation. It keys handshake
/// state on the caller, so without this a second page load meets a
/// connection still relaying the first, and its greeting is carried to the
/// exit as stream bytes rather than starting a handshake.
const STREAM_RESET: u8 = 1;

/// Stream bytes in a numbered exchange: marker, u32 number, bytes. A proxy
/// takes exit bytes out of its inbox to answer, and holds a request for up
/// to a second while the exit is slow, but a poll here waits 60 ms and the
/// kernel drops a reply that arrives after that: the bytes in it were gone.
/// Asking again with the same number gets the same answer instead.
const STREAM_NUMBERED: u8 = 2;

/// One exchange with the proxy, marker on and marker off.
fn ask(port: u32, payload: &[u8]) -> Result<Vec<u8>, ()> {
    let seq = with(|route| route.seq)?;
    let mut framed = Vec::with_capacity(5 + payload.len());
    framed.push(STREAM_NUMBERED);
    framed.extend_from_slice(&seq.to_le_bytes());
    framed.extend_from_slice(payload);
    let answer = exchange(port, &framed, payload.is_empty())?;
    with(|route| route.seq = route.seq.wrapping_add(1).max(1))?;
    Ok(answer)
}

/// Take the marker off an answer and record what it said about the tunnel.
fn absorb(reply: &[u8]) -> Result<(), ()> {
    let Some((&marker, body)) = reply.split_first() else {
        return Err(());
    };
    with(|route| {
        route.pending.extend_from_slice(body);
        route.closed = marker == STREAM_CLOSED;
    })
}

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
    // Nothing held and the tunnel still open means the answer may simply not
    // have arrived yet, so ask. A closed tunnel has nothing more to give and
    // asking again would only stall the reader.
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

pub fn close() -> bool {
    with(|route| {
        route.pending.clear();
        route.closed = false;
    })
    .is_ok()
}
