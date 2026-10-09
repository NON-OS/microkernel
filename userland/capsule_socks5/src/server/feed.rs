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

use super::open::{open_tunnel, OpenOutcome};
use super::relay::relay;
use super::reply::Reply;
use super::state::{Server, SERVER};
use super::who::Who;
use crate::conn::Event;
use crate::wire::method_reply;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicI64, Ordering};

/// Drive the SOCKS handshake with bytes from the client, returning whatever
/// should go back.
///
/// A rejected or malformed client is answered and closed rather than left
/// half open: the state machine reports that through `is_closed`, and holding
/// the slot would deny it to the next caller. `room` bounds the stream bytes
/// a relayed answer brings back.
pub fn feed(pid: Who, data: &[u8], room: usize) -> Reply {
    let mut guard = SERVER.lock();
    let Some(server) = guard.as_mut() else {
        return Reply::closed(Vec::new());
    };
    // A full table may be full of callers that ended.
    if server.clients.get(pid).is_none() {
        free_ended(server);
    }
    let Some(conn) = server.clients.get(pid) else {
        // Table full. Refusing is the honest answer: the client disconnects
        // rather than waiting on a handshake that will never advance.
        return Reply::closed(method_reply(false).to_vec());
    };
    match conn.on_client(data) {
        Event::NeedMore => Reply::open(Vec::new()),
        Event::ToClient { buf, len } => {
            let bytes = buf[..len].to_vec();
            // A rejection answers and then closes, so the caller is told both
            // at once rather than being left to poll a dead connection.
            match server.clients.get(pid).map(|c| c.is_closed()) {
                Some(true) | None => Reply::closed(bytes),
                Some(false) => Reply::open(bytes),
            }
        }
        Event::Relay => {
            let bytes = relay(server, pid, data, room);
            match server.manager.id_of_socket(pid) {
                Some(_) => Reply::open(bytes),
                None => Reply::closed(bytes),
            }
        }
        Event::Close => {
            if let Some(conn) = server.manager.close_socket(pid) {
                server.inbox.forget(conn);
            }
            server.clients.drop_client(pid);
            Reply::closed(Vec::new())
        }
        Event::Open(dest) => {
            // A full table has no id to give. Zero is reserved for "no
            // connection", so using it would open a tunnel that every later
            // frame fails to match, rather than telling the client no.
            let outcome = match server.manager.open(pid) {
                Some(id) => open_tunnel(id, &dest),
                None => OpenOutcome::BadRequest,
            };
            let opened = outcome == OpenOutcome::Opened;
            let Some(conn) = server.clients.get(pid) else {
                return Reply::closed(Vec::new());
            };
            let (reply, len) = conn.opened(outcome.reply_code());
            let bytes = reply[..len].to_vec();
            if opened {
                Reply::open(bytes)
            } else {
                Reply::closed(bytes)
            }
        }
    }
}

fn alive(pid: u32) -> bool {
    nonos_libc::mk_pid_alive(pid)
}

/// Free the slot, tunnel and held bytes of every caller that ended, as its
/// own reset would have.
fn free_ended(server: &mut Server) {
    for pid in server.clients.ended(alive) {
        if let Some(conn) = server.manager.close_socket(pid) {
            server.inbox.forget(conn);
        }
        server.clients.drop_client(pid);
    }
}

const REAP_GAP_MS: i64 = 2_000;

static LAST_REAP: AtomicI64 = AtomicI64::new(i64::MIN);

/// Free every ended caller at most every REAP_GAP_MS, with the replies kept
/// for them.
pub fn reap_due() {
    let now = nonos_libc::mk_uptime_ms();
    let last = LAST_REAP.load(Ordering::Relaxed);
    if last != i64::MIN && now >= last && now - last < REAP_GAP_MS {
        return;
    }
    LAST_REAP.store(now, Ordering::Relaxed);
    if let Some(server) = SERVER.lock().as_mut() {
        free_ended(server);
    }
    super::kept::forget_ended(alive);
}

/// Whether a conversation of `pid`'s is held.
pub fn holds(pid: Who) -> bool {
    SERVER.lock().as_ref().is_some_and(|s| s.clients.holds(pid))
}

/// Forget a caller's conversation so its next request starts a fresh one.
///
/// Handshake state is keyed on the caller, so a second request from the same
/// capsule would otherwise arrive at a connection already relaying, and its
/// greeting would be carried to the exit as stream bytes.
pub fn reset_client(pid: Who) -> Reply {
    let mut guard = SERVER.lock();
    let Some(server) = guard.as_mut() else {
        return Reply::closed(Vec::new());
    };
    if let Some(conn) = server.manager.close_socket(pid) {
        server.inbox.forget(conn);
    }
    server.clients.drop_client(pid);
    Reply::open(Vec::new())
}
