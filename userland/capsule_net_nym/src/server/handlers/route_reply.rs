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
use spin::Mutex;

use crate::reply::{open_reply, reply_message, Collected, Reassembly, Reply};
use crate::sphinx::constants::ACK_PLAINTEXT_SIZE;
use crate::state::{RX_DEPTH, TABLE};
use crate::surb::keys_held;
use crate::trace;

/// Replies being rebuilt from their fragments.
///
/// Many at once: an exit answers a page as dozens of messages sent back to
/// back, and their packets arrive interleaved. A set id is only trusted this
/// far because the fragment carrying it opened under one of our reply block
/// keys, which nobody but the exit we handed them to can use.
static PENDING: Mutex<Reassembly> = Mutex::new(Reassembly::new());

/// Take a message the gateway pushed and deliver what it turns out to be.
///
/// Everything here is a filter, and a message that fails any of them is
/// dropped rather than passed on. A gateway pushes whatever it is handed, so
/// arriving is not evidence of anything: only opening under one of the reply
/// block keys we handed out is.
pub fn route_reply(tcp_port: u32, payload: &[u8]) {
    trace::say_num(b"pushed message bytes", payload.len() as u64);
    // An acknowledgement is a fragment id under its own iv and nothing else,
    // so its width names it. It is not a reply and will not open as one. What
    // it says is that the fragment it names arrived, which is the only word
    // the far end sends back unprompted.
    if payload.len() == ACK_PLAINTEXT_SIZE {
        if crate::server::acknowledge(payload) {
            trace::say_num(b"fragment acknowledged, still waiting on", crate::server::waiting() as u64);
        } else {
            trace::say(b"fragment acknowledged again, or not one of ours");
        }
        return;
    }
    let Some(fragment) = open_reply(payload) else {
        // Either it was not sealed to one of our blocks, or it is shorter
        // than the parts a reply is read in. Both mean it was not for us.
        trace::say_num(b"push dropped: no reply block key matched, keys held", keys_held() as u64);
        return;
    };
    // It came back on one of our blocks, so the far end holds one fewer.
    TABLE.lock().with_sphinx_session(|session| session.surbs.spent());
    let mut pending = PENDING.lock();
    let message = match pending.collect(&fragment, mk_uptime_ms()) {
        Collected::Complete(message) => message,
        Collected::Held => {
            trace::say_two(
                b"push held: sets waiting, bytes",
                pending.pending() as u64,
                pending.held_bytes() as u64,
            );
            return;
        }
        // The far end resends a fragment whose acknowledgement was slow, so a
        // copy of one already placed or delivered is normal and harmless.
        Collected::Duplicate => {
            trace::say(b"push dropped: a copy of a fragment already taken");
            return;
        }
        Collected::Refused => {
            trace::say_num(
                b"push dropped: not a fragment this will hold, bytes",
                fragment.len() as u64,
            );
            return;
        }
    };
    drop(pending);
    match reply_message(&message) {
        Some(Reply::Data(body)) => {
            trace::say_num(b"reply delivered bytes", body.len() as u64);
            deliver(body);
            super::top_up::send_ahead(tcp_port);
        }
        // The far end has spent down to the reserve it keeps and will say
        // nothing more until it has room to answer. Everything sent after
        // this point depends on the top up going out.
        Some(Reply::SurbRequest { recipient, amount }) => {
            trace::say_num(b"far end asked for reply blocks", amount as u64);
            super::top_up::answer_request(tcp_port, &recipient, amount);
        }
        None => {
            trace::say_num(b"push dropped: not a reply message, bytes", message.len() as u64);
        }
    }
}

/// Hand a reply to the session waiting on one.
///
/// A reply names no session: it came back on a block we handed out, and the
/// session that handed it out is the one holding a destination.
fn deliver(body: &[u8]) {
    let mut owned = Vec::with_capacity(body.len());
    owned.extend_from_slice(body);
    let backlog = TABLE.lock().with_sphinx_session(|session| {
        session.push(owned);
        session.backlog()
    });
    // Half full means the reader has stopped collecting, and past full the
    // oldest message goes: worth a line before that happens, not after.
    if let Some((messages, bytes)) = backlog {
        if messages > RX_DEPTH / 2 {
            trace::say_two(
                b"reader falling behind: messages, bytes",
                messages as u64,
                bytes as u64,
            );
        }
    }
}
