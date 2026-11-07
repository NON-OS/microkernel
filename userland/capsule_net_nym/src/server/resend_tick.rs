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

//! Sending again what the far end has not acknowledged.
//!
//! See `ack/ledger.rs` for why: a message reaches an exit only whole, and a
//! request carrying its reply blocks is a dozen packets or more. Every packet
//! that leaves for a recipient is recorded here, every acknowledgement that
//! comes home takes its fragment off, and on each turn of the loop whatever
//! has waited past its time is sealed again on a fresh route and sent.

use alloc::vec::Vec;
use nonos_libc::mk_uptime_ms;
use spin::Mutex;

use crate::ack::ledger::{next_wait_ms, Ledger, Resend, LEDGER_RECIPIENT};
use crate::ack::open_ack;
use crate::mixnet::{encode_fragment, Addressed, Encoded};
use crate::server::handlers::ready;
use crate::state::{ack_key, Gateway, Session, TABLE};
use crate::trace;

static LEDGER: Mutex<Ledger> = Mutex::new(Ledger::new());

/// Who a session's messages are for: identity, encryption key, gateway.
pub fn recipient_of(session: &Session) -> [u8; LEDGER_RECIPIENT] {
    let mut out = [0u8; LEDGER_RECIPIENT];
    out[..32].copy_from_slice(&session.dest);
    out[32..64].copy_from_slice(&session.dest_encryption);
    out[64..].copy_from_slice(&session.dest_gateway);
    out
}

/// Record fragments that have just gone out to `recipient` for the first
/// time.
pub fn record(recipient: &[u8; LEDGER_RECIPIENT], sent: Vec<Encoded>) {
    let now = mk_uptime_ms();
    let mut ledger = LEDGER.lock();
    let mut evicted = 0;
    for e in sent {
        evicted += ledger.sent(e.frag_id, *recipient, e.fragment, now, e.wait_ms, 1);
    }
    if evicted != 0 {
        trace::say_two(b"resend: ledger full, let go, bytes held", evicted as u64, ledger.bytes() as u64);
    }
}

/// An acknowledgement came home. Returns whether it named a fragment still
/// waiting on one.
pub fn acknowledge(payload: &[u8]) -> bool {
    let Some(key) = ack_key() else {
        return false;
    };
    let Some(id) = open_ack(&key, payload) else {
        return false;
    };
    LEDGER.lock().acked(&id)
}

/// Fragments still waiting on an acknowledgement.
pub fn waiting() -> usize {
    LEDGER.lock().len()
}

/// Send again whatever has waited past its time.
pub fn resend_tick() {
    let now = mk_uptime_ms();
    {
        let ledger = LEDGER.lock();
        if ledger.is_empty() || ledger.next_due().is_some_and(|due| due > now) {
            return;
        }
    }
    let mut table = TABLE.lock();
    if table.sphinx_session_count() == 0 {
        // No session is bound to a recipient any more, so nothing held can
        // be for one that is still being talked to.
        let mut dropped = Vec::new();
        let swept = LEDGER.lock().due(now, None, &mut dropped);
        drop(table);
        if swept.stale != 0 {
            trace::say_num(b"resend: session ended, fragments let go", swept.stale as u64);
        }
        return;
    }
    let built = table.with_sphinx_session(|session| build(session, now));
    drop(table);
    let Some((_via, recipient, packets)) = built.flatten() else {
        return;
    };
    send(&recipient, packets);
}

/// A resend sealed and ready, with what its record needs once it has left.
struct Ready {
    resend: Resend,
    packet: Vec<u8>,
    wait_ms: i64,
}

type Built = Option<(Gateway, [u8; LEDGER_RECIPIENT], Vec<Ready>)>;

/// Seal everything due for this session's recipient. Built while the table
/// is held and sent once it is released, as a top up is.
fn build(session: &Session, now: i64) -> Built {
    let recipient = recipient_of(session);
    let mut due = Vec::new();
    let swept = LEDGER.lock().due(now, Some(&recipient), &mut due);
    if swept.stale != 0 {
        trace::say_num(b"resend: recipient changed, fragments let go", swept.stale as u64);
    }
    if swept.given_up != 0 {
        trace::say_num(b"resend: fragments out of tries, given up", swept.given_up as u64);
    }
    if due.is_empty() {
        return None;
    }
    let Ok(prepared) = ready(session, 0) else {
        // Nothing can be sealed now; the fragments stay out of the ledger,
        // and the stream they belonged to ends on its own silence.
        trace::say_num(b"resend: could not prepare, fragments let go", due.len() as u64);
        return None;
    };
    let mut identity = [0u8; 32];
    let mut encryption = [0u8; 32];
    let mut gateway = [0u8; 32];
    identity.copy_from_slice(&recipient[..32]);
    encryption.copy_from_slice(&recipient[32..64]);
    gateway.copy_from_slice(&recipient[64..]);
    let addressed = Addressed {
        destination: &identity,
        destination_encryption: &encryption,
        destination_gateway: &gateway,
        our_identity: &prepared.identity,
        ack_key: &prepared.ack_key,
        home: &prepared.home,
        sender_tag: &session.sender_tag,
        reply_surbs: &[],
    };
    let mut out = Vec::with_capacity(due.len());
    for resend in due {
        let Some((packet, fresh_ms)) = encode_fragment(&addressed, resend.frag_id, &resend.fragment)
        else {
            continue;
        };
        let wait_ms = next_wait_ms(resend.wait_ms).max(fresh_ms);
        out.push(Ready { resend, packet, wait_ms });
    }
    Some((session.gateway, recipient, out))
}

fn send(recipient: &[u8; LEDGER_RECIPIENT], packets: Vec<Ready>) {
    let total = packets.len();
    let mut queued = 0usize;
    // Each fragment is its own job: it is recorded again when it leaves, and
    // a full queue keeps the rest for the next turn of the ledger.
    for r in packets {
        let done = crate::server::Done::Resent {
            recipient: *recipient,
            resend: r.resend,
            wait_ms: r.wait_ms,
        };
        if crate::server::enqueue(None, alloc::vec![crate::server::Frame::Sphinx(r.packet)], done)
            != crate::protocol::E_OK
        {
            break;
        }
        queued += 1;
    }
    trace::say_two(b"resend: fragments queued again, of due", queued as u64, total as u64);
}

/// A resent fragment has left: it waits on its acknowledgement again.
pub fn resent(recipient: &[u8; LEDGER_RECIPIENT], resend: Resend, wait_ms: i64) {
    let sends = resend.sends.saturating_add(1);
    LEDGER.lock().sent(resend.frag_id, *recipient, resend.fragment, mk_uptime_ms(), wait_ms, sends);
}
