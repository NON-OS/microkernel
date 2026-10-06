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
use nonos_libc::mk_uptime_ms;

use crate::gateway_client;
use crate::message::repliable_additional_surbs;
use crate::mixnet::{encode_built, Addressed, Encoded};
use crate::reply::RECIPIENT_BYTES;
use crate::state::{Gateway, Session, TABLE};
use crate::surb::build_supply;
use crate::trace;

use super::send_ready::ready;

/// Send more reply blocks to a recipient that has run low and asked.
///
/// A recipient keeps a reserve it will not spend, and once it is down to that
/// it stops answering and asks instead. Until it is topped up nothing further
/// arrives, which looks exactly like a transfer that stalled: the first part
/// of a page lands and the rest never does.
///
/// It names how many it wants. It is sent at least enough to leave it room
/// past its reserve and at most what one of its own requests may ask for;
/// forty, the old ceiling, was less than half of what a 100 KB answer
/// needs, so a large answer stalled at every reserve.
pub fn answer_request(tcp_port: u32, recipient: &[u8; RECIPIENT_BYTES], amount: u32) {
    let built = TABLE.lock().with_sphinx_session(|session| {
        let count = session.surbs.asked(amount);
        let out = build(session, recipient, count)?;
        session.surbs.sent(count);
        Some((out, session.surbs.held()))
    });
    send(tcp_port, built, b"top up: blocks asked for, then held");
}

/// Send reply blocks before the far end has to ask.
///
/// Consulted as each answer arrives. Asking costs the far end a whole round
/// trip in which it sends nothing, every time it reaches its reserve; blocks
/// sent while it is still answering arrive before it gets there.
pub fn send_ahead(tcp_port: u32) {
    let now = mk_uptime_ms();
    let built = TABLE.lock().with_sphinx_session(|session| {
        let count = session.surbs.top_up_due(now)?;
        let mut recipient = [0u8; RECIPIENT_BYTES];
        recipient[..32].copy_from_slice(&session.dest);
        recipient[32..64].copy_from_slice(&session.dest_encryption);
        recipient[64..].copy_from_slice(&session.dest_gateway);
        let out = build(session, &recipient, count)?;
        session.surbs.topped_up(count, now);
        Some((out, session.surbs.held()))
    });
    send(tcp_port, built, b"top up: blocks ahead of need, then held");
}

/// The packets carrying `count` new reply blocks to `recipient`.
///
/// Built while the table is held and sent once it is released. Sending
/// reaches for the table again if the link turns out to be gone, and a
/// session deliberately cannot be copied out, so the work is split either
/// side of the lock rather than the session being moved across it.
fn build(
    session: &Session,
    recipient: &[u8; RECIPIENT_BYTES],
    count: u32,
) -> Option<(Gateway, [u8; RECIPIENT_BYTES], usize, Vec<Encoded>)> {
    let mut identity = [0u8; 32];
    let mut encryption = [0u8; 32];
    let mut gateway = [0u8; 32];
    identity.copy_from_slice(&recipient[..32]);
    encryption.copy_from_slice(&recipient[32..64]);
    gateway.copy_from_slice(&recipient[64..96]);

    let prepared = ready(session, 0).ok()?;
    let surbs = build_supply(&session.gateway.identity, &prepared.identity, count as usize)?;
    let addressed = Addressed {
        destination: &identity,
        destination_encryption: &encryption,
        destination_gateway: &gateway,
        our_identity: &prepared.identity,
        ack_key: &prepared.ack_key,
        home: &prepared.home,
        sender_tag: &session.sender_tag,
        reply_surbs: &surbs,
    };
    let message = repliable_additional_surbs(&session.sender_tag, &surbs);
    let packets = encode_built(&addressed, message)?;
    Some((session.gateway, *recipient, surbs.len(), packets))
}

/// What a top up needs on the wire: the gateway, who it is for, how many
/// blocks, the packets, and how many the far end is then believed to hold.
type Built = Option<Option<((Gateway, [u8; RECIPIENT_BYTES], usize, Vec<Encoded>), u32)>>;

fn send(tcp_port: u32, built: Built, what: &[u8]) {
    let Some(Some(((via, recipient, sent, packets), held))) = built else {
        return;
    };
    trace::say_two(what, sent as u64, held as u64);
    for packet in &packets {
        let Ok(frame) =
            gateway_client::make_encrypted_blob(gateway_client::KIND_FORWARD_SPHINX, &packet.packet)
        else {
            return;
        };
        if gateway_client::send(tcp_port, via, &frame).is_err() {
            crate::server::gateway_lost();
            return;
        }
    }
    // Reply blocks lost on the way are blocks the far end believes it was
    // never sent, and a stream that stalls at its reserve: held until each
    // fragment is acknowledged, like any other message.
    crate::server::record(&recipient, packets);
}
