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

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_uptime_ms;

use super::gather::{gather, Mixnet, Received};
use super::inbox::{Accept, Inbox};
use super::state::Server;
use super::who::Who;
use crate::manager::Manager;
use crate::nym::{recv_batch, send_through_mixnet, Delivery, SendError};
use crate::tunnel::{decode_response, encode_send, SEND_DATA_MAX, SEND_FRAME_MAX};

/// How long one answer waits on the exit when nothing has come back.
///
/// Shorter than the browser waits on a poll (60 ms), so the answer reaches
/// it rather than a caller that has stopped listening. A whole second was
/// once needed here because a reply arriving between polls was lost; now
/// nothing is: net.nym queues every reply as it lands, the inbox holds what
/// it has read, and an answer the caller missed is kept and given again. So
/// waiting longer only made the browser miss the answer and ask again for
/// the same one, a round trip per message.
const HOLD_MS: i64 = 40;

/// Carry `data` to the exit and bring back whatever has come the other way.
///
/// An empty `data` is a read with nothing to send, which is how a client asks
/// whether the far end has answered yet. At most `room` bytes come back.
pub fn relay(server: &mut Server, pid: Who, data: &[u8], room: usize) -> Vec<u8> {
    rotate_if_stalled(server);
    let Some(conn) = server.manager.id_of_socket(pid) else {
        return Vec::new();
    };
    if !data.is_empty() && !forward(server, conn, data) {
        // A send that did not leave is a number the exit waits on forever,
        // so the stream cannot go on. Ending it here has the client
        // reconnect at once instead of waiting out an answer that cannot
        // come.
        server.inbox.forget(conn);
        server.manager.close(conn);
        return Vec::new();
    }
    collect(server, conn, room)
}

/// Walk off an exit that has answered nothing since we started sending.
///
/// A directory lists exits whose requesters may not serve traffic, and one
/// that answers the lookup but never a request wedges every connection bound
/// to it. When the watch says the silence budget is spent, the exit and the
/// session bound to it are dropped, and every live connection is ended from
/// this side: the client reconnects immediately and the new connect opens a
/// session against the next exit in the directory's list.
fn rotate_if_stalled(server: &mut Server) {
    if !crate::nym::rotate_if_silent() {
        return;
    }
    crate::server::trace_open(b"exit silent, rotating to the next", 0);
    crate::nym::reset_session();
    for id in server.manager.open_ids() {
        server.inbox.close_now(id);
    }
}

/// Carry `data` to the exit as numbered sends, each within one mix payload.
fn forward(server: &mut Server, conn: u64, data: &[u8]) -> bool {
    let mut buf = vec![0u8; SEND_FRAME_MAX];
    for piece in data.chunks(SEND_DATA_MAX) {
        let Some(seq) = server.manager.next_seq(conn) else {
            return false;
        };
        let Some(n) = encode_send(conn, seq, false, piece, &mut buf) else {
            return false;
        };
        if let Err(e) = send_through_mixnet(&buf[..n]) {
            let code = match e {
                SendError::Remote(code) => code,
                SendError::NoExit | SendError::NoSession | SendError::TooLarge => 0,
            };
            crate::server::trace_open(b"send refused, ending the stream", code);
            return false;
        }
    }
    true
}

fn collect(server: &mut Server, conn: u64, room: usize) -> Vec<u8> {
    // No room is an answer already full of bytes the caller missed; what has
    // come back since waits for the next one.
    if room == 0 {
        return Vec::new();
    }
    let Server { inbox, manager, .. } = server;
    inbox.tick(mk_uptime_ms());
    let got = gather(inbox, conn, room, HOLD_MS, &mut Live, |inbox, msg| take(inbox, manager, msg));
    if got.closed {
        inbox.forget(conn);
        manager.close(conn);
        return got.bytes;
    }
    // The session every stream rides on is gone, so none of them will hear
    // another byte. Each is ended now, and its reader told, the way an exit
    // rotation ends them; the next connection opens a fresh session.
    if got.gone && crate::nym::session().is_none() {
        crate::server::trace_step(b"mixnet session lost, streams ended", manager.count() as u64);
        for id in manager.open_ids() {
            inbox.close_now(id);
        }
    }
    // A message in front of bytes already here has not come in the time the
    // exit takes to resend one. It will not come now, and the stream cannot
    // move past it, so it is ended and the reader told rather than left to
    // wait out its own patience on a stream that is already dead.
    inbox.tick(mk_uptime_ms());
    if let Some(missing) = inbox.stalled(conn) {
        crate::server::trace_step(b"stream stalled, never came: message", missing);
        inbox.close_now(conn);
    }
    got.bytes
}

/// The mixnet as net.nym presents it.
struct Live;

impl Mixnet for Live {
    fn receive(&mut self, wait_ms: u32) -> Received {
        match recv_batch(wait_ms) {
            Delivery::Messages(messages) => Received::Messages(messages),
            Delivery::Empty => Received::Empty,
            Delivery::Gone => Received::Gone,
        }
    }

    fn now_ms(&self) -> i64 {
        mk_uptime_ms()
    }
}

/// File one delivered message against the connection it names.
///
/// A message that does not decode is dropped rather than guessed at. The
/// mixnet delivers whatever was addressed to us, and arriving is not evidence
/// that it belongs to a connection of ours. Nor is one for a connection that
/// has already ended: the exit keeps sending until it hears the close, and
/// holding what it sent for a reader that is gone kept it for ever.
fn take(inbox: &mut Inbox, manager: &Manager, msg: &[u8]) {
    crate::server::trace_reply_bytes(msg.len());
    let Some(response) = decode_response(msg) else {
        // It arrived and was not ours to read. Saying so separates a reply
        // the exit never sent from one sent in a shape we do not speak.
        crate::server::trace_reply_kind(msg);
        return;
    };
    if manager.socket_of(response.conn_id).is_none() {
        return;
    }
    // Only stream payload proves the exit. A requester can acknowledge a
    // connect and still never carry a byte back; counting those control
    // frames as delivery once pinned a session to exactly such a node and
    // turned the rotation off for it.
    if !response.data.is_empty() {
        crate::nym::note_delivered();
    } else {
        crate::nym::note_answered();
    }
    match inbox.accept(response.conn_id, response.seq, response.closed, response.data) {
        Accept::Held | Accept::Duplicate => {}
        Accept::Overflow(ended) => {
            crate::server::trace_step(b"reader fell behind, stream ended", ended);
            crate::server::trace_step(b"inbox holds bytes", inbox.held_bytes() as u64);
        }
    }
}
