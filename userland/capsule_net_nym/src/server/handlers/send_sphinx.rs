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

use crate::mixnet::{encode_message, Addressed};
use crate::protocol::{E_NO_ROUTE, E_OK};
use crate::server::handlers::send_ready::ready;
use crate::server::{Done, Frame};
use crate::state::Session;
use crate::trace;

/// Send a request through the mixnet as a real message.
///
/// Refuses rather than degrades: a message that cannot be answered or that
/// would be linkable is not sent at all.
pub fn send_sphinx(_tcp_port: u32, session: &mut Session, payload: &[u8]) -> u16 {
    let prepared = match ready(session, session.surbs.for_request() as usize) {
        Ok(prepared) => prepared,
        Err(errno) => return errno,
    };

    let addressed = Addressed {
        destination: &session.dest,
        destination_encryption: &session.dest_encryption,
        destination_gateway: &session.dest_gateway,
        our_identity: &prepared.identity,
        ack_key: &prepared.ack_key,
        home: &prepared.home,
        sender_tag: &session.sender_tag,
        reply_surbs: &prepared.reply_surbs,
    };
    let Some(packets) = encode_message(&addressed, payload) else {
        trace::say_num(b"send refused: could not build packets for bytes", payload.len() as u64);
        return E_NO_ROUTE;
    };
    trace::say_two(b"sending packets for bytes", packets.len() as u64, payload.len() as u64);

    // Out when the sending clock says (server/outbound). The reply blocks
    // count as the far end's, and the fragments wait on acknowledgements,
    // once the last of them has left.
    let mut packets = packets;
    let frames =
        packets.iter_mut().map(|p| Frame::Sphinx(core::mem::take(&mut p.packet))).collect();
    let done = Done::Message {
        pid: session.owner,
        id: session.id,
        recipient: crate::server::recipient_of(session),
        fragments: packets,
        surbs: prepared.reply_surbs.len() as u32,
    };
    let errno = crate::server::enqueue(Some((session.owner, session.id)), frames, done);
    if errno != E_OK {
        trace::say(b"send refused: the send queue is full");
        return errno;
    }
    trace::say(b"queued");
    E_OK
}
